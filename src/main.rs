extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate termion;

use std::{io, thread};
use std::io::{stdout, Stdout, Write};

use termion::color;
use termion::event::Key;
use termion::raw::{IntoRawMode, RawTerminal};

use crate::arkanoid::arkanoidmain::ArkanoidMain;
use crate::common::persistence::HighScores;
use crate::common::ioutils::wait_for_key;
use crate::snake::snakemain::SnakeMain;
use crate::tetris::tetrismain::TetrisMain;
use crate::wator::watormain::WatorMain;
use std::time::Duration;

mod arkanoid;
mod common;
mod snake;
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

pub trait Main<W: Write> {
    fn name(&self) -> &'static str;

    fn run(&self, stdout: &mut W) -> io::Result<Option<u32>>;

    fn high_scores(&self) -> io::Result<HighScores>;
}

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    loop {
        write!(stdout,
               "{}{}{}{}Console games{}\r\n\n",
               termion::cursor::Hide,
               termion::clear::All,
               termion::cursor::Goto(1, 1),
               color::Bg(color::Red),
               termion::style::Reset).unwrap();

        let mains: Vec<Box<dyn Main<RawTerminal<Stdout>>>> =
            vec!(Box::new(TetrisMain::new()),
                 Box::new(SnakeMain::new()),
                 Box::new(WatorMain::new()),
                 Box::new(ArkanoidMain::new())
            );

        let menu = mains.iter().map(|main| main.name()).collect();

        let choice = common::menu::choose(&mut stdout, &menu, 1, 3).unwrap();

        if let Some(index) = choice {
            attempt! {{
            run(&mut stdout, mains.into_iter().enumerate().find(|(i, _main)| *i == index as usize).unwrap().1);
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

fn run<W>(stdout: &mut W, main: Box<dyn Main<W>>) -> io::Result<()> where W: Write {
    let scores = main.high_scores()?;

    write!(stdout,
           "{}{}High scores",
           termion::clear::All,
           termion::cursor::Goto(10, 1))?;

    let mut y = 3;
    for score in scores.entries().iter() {
        write!(stdout,
               "{}{}",
               termion::cursor::Goto(10, y),
               score.score())?;
        y += 1;
    }

    write!(stdout,
           "{}Press s to start.",
           termion::cursor::Goto(1, 20))?;

    stdout.flush()?;

    wait_for_key(Key::Char('s'))?;

    let result = main.run(stdout)?;

    if let Some(score) = result {
        let mut scores = main.high_scores()?;
        // game is ended
        scores.add(score);
        scores.save()?;

        write!(stdout,
               "{}{}Game over! Score: {}  \n\r\n\rPress c to continue.",
               termion::clear::All,
               termion::cursor::Goto(1, 10),
               score)?;

        stdout.flush()?;

        wait_for_key(Key::Char('c'))?;

        Ok(())
    } else {
        Result::Ok(())
    }
}
