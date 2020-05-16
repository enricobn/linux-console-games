extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate termion;

use std::io;
use std::io::{Read, stdout, Stdout, Write};

use chrono::{DateTime, Local};
use termion::{async_stdin, AsyncReader, color};
use termion::event::Key;
use termion::raw::{IntoRawMode, RawTerminal};

use crate::arkanoid::arkanoidmain::ArkanoidMain;
use crate::common::ioutils::wait_for_key_async;
use crate::common::persistence::HighScores;
use crate::snake::snakemain::SnakeMain;
use crate::spaceinvaders::spaceinvadersmain::SpaceInvadersMain;
use crate::tetris::tetrismain::TetrisMain;
use crate::wator::watormain::WatorMain;

mod arkanoid;
mod common;
mod snake;
mod spaceinvaders;
mod tetris;
mod wator;

// from https://stackoverflow.com/questions/55755552/what-is-the-rust-equivalent-to-a-try-catch-statement
macro_rules! attempt { // `try` is a reserved keyword
   (@recurse ($a:expr) { } catch ($e:ident) $b:block) => {
      if let Err ($e) = $a $b
   };
   (@recurse ($a:expr) { $e:expr; $($tail:tt)* } $($handler:tt)*) => {
      attempt!{@recurse ($a.and_then (|_| $e)) { $($tail)* } $($handler)*}
   };
   ({ $e:expr; $($tail:tt)* } $($handler:tt)*) => {
      attempt!{@recurse ($e) { $($tail)* } $($handler)* }
   };
}

pub trait Main<W: Write, R: Read> {
    fn name(&self) -> &'static str;

    fn run(&self, stdout: &mut W, stdin: &mut R) -> io::Result<Option<u32>>;

    fn high_scores(&self) -> io::Result<HighScores>;
}

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin();

    loop {
        write!(stdout,
               "{}{}{}{}Console games{}\r\n\n",
               termion::cursor::Hide,
               termion::clear::All,
               termion::cursor::Goto(1, 1),
               color::Bg(color::Red),
               termion::style::Reset).unwrap();

        let mains: Vec<Box<dyn Main<RawTerminal<Stdout>, AsyncReader>>> =
            vec!(Box::new(TetrisMain::new()),
                 Box::new(SnakeMain::new()),
                 Box::new(WatorMain::new()),
                 Box::new(ArkanoidMain::new()),
                 Box::new(SpaceInvadersMain::new())
            );

        let menu = mains.iter().map(|main| main.name()).collect();

        let choice = common::menu::choose(&mut stdout, &mut stdin, &menu, 1, 3).unwrap();

        if let Some(index) = choice {
            attempt! {{
            run(&mut stdout, &mut stdin, mains.into_iter().enumerate().find(|(i, _main)| *i == index as usize).unwrap().1);
        } catch(e) {
            write!(stdout,
                   "{}\n\r",
                   termion::cursor::Show)
                .unwrap();

            stdout.flush().unwrap();

            println!("Failed to run: {}", e);
        }}
        } else {
            break;
        }
    }

    write!(stdout,
           "{}\r\n",
           termion::cursor::Show).unwrap();
}

fn run<W, R>(stdout: &mut W, stdin: &mut R, main: Box<dyn Main<W, R>>) -> io::Result<()> where W: Write, R: Read {
    let scores = main.high_scores()?;

    print_scores(stdout, scores, None)?;

    write!(stdout,
           "{}Press s to start.",
           termion::cursor::Goto(1, 20))?;

    stdout.flush()?;

    wait_for_key_async(stdin, Key::Char('s'))?;

    let result = main.run(stdout, stdin)?;

    if let Some(score) = result {
        let mut scores = main.high_scores()?;

        let added = scores.add(score);

        scores.save()?;

        print_scores(stdout, scores, added.map(|score| score.time()))?;

        write!(stdout,
               "{}Game over! Score: {}  \n\r\n\rPress c to continue.",
               termion::cursor::Goto(1, 15),
               score)?;

        stdout.flush()?;

        wait_for_key_async(stdin, Key::Char('c'))?;

        Ok(())
    } else {
        Result::Ok(())
    }
}

fn print_scores<W: Write>(stdout: &mut W, scores: HighScores, highlight: Option<DateTime<Local>>) -> io::Result<()> {
    write!(stdout,
           "{}{}High scores",
           termion::clear::All,
           termion::cursor::Goto(10, 1))?;

    let mut y = 3;
    for score in scores.entries().iter() {
        if Some(score.time()) == highlight {
            write!(stdout, "{}",
                   color::Fg(color::Green))?;
        } else {
            write!(stdout, "{}",
                   termion::style::Reset)?;
        }
        write!(stdout,
               "{}{}",
               termion::cursor::Goto(10, y),
               score.score())?;
        y += 1;
    }
    Ok(())
}
