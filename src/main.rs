extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate termion;

use std::{io, thread};
use std::io::{Read, stdout, Write};
use std::time::Duration;

use chrono::{DateTime, Local};
use termion::{async_stdin, color};
use termion::event::Key;
use termion::event::Key::Char;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

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

    attempt! {{
        run(&mut stdout, &mut stdin);
    } catch(e) {
        reset_status(&mut stdout).unwrap();

        println!("Failed to run: {}", e);
    }};

    reset_status(&mut stdout).unwrap();
}

fn reset_status<W: 'static>(stdout: &mut W) -> io::Result<()> where W: Write {
    write!(stdout,
           "{}{}\r\n",
           termion::cursor::Show,
           termion::style::Reset)?;
    stdout.flush()
}

fn run<W: 'static, R: 'static>(stdout: &mut W, stdin: &mut R) -> io::Result<()> where W: Write, R: Read {
    loop {
        write!(stdout,
               "{}{}{}{}{}Console games{}\r\n\r\nPress {}Esc{} to exit",
               termion::cursor::Hide,
               termion::clear::All,
               termion::cursor::Goto(1, 1),
               color::Fg(color::LightWhite),
               color::Bg(color::Green),
               termion::style::Reset,
               color::Fg(color::LightWhite),
               termion::style::Reset).unwrap();

        let mains: Vec<Box<dyn Main<W, R>>> =
            vec!(Box::new(TetrisMain::new()),
                 Box::new(SnakeMain::new()),
                 Box::new(WatorMain::new()),
                 Box::new(ArkanoidMain::new()),
                 Box::new(SpaceInvadersMain::new())
            );

        let menu = mains.iter().map(|main| main.name()).collect();

        let choice = common::menu::choose(stdout, stdin, &menu, 1, 5).unwrap();

        if let Some(index) = choice {
            run_main(stdout, stdin, mains.into_iter().enumerate().find(|(i, _main)| *i == index as usize).unwrap().1)?;
        } else {
            break;
        }
    }

    Ok(())
}

fn run_main<W, R>(stdout: &mut W, stdin: &mut R, main: Box<dyn Main<W, R>>) -> io::Result<()> where W: Write, R: Read {
    let scores = main.high_scores()?;

    print_scores(stdout, scores, None)?;

    write!(stdout,
           "{}Press {}p{} to play.",
           termion::cursor::Goto(1, 20),
           color::Fg(color::LightWhite),
           termion::style::Reset)?;

    stdout.flush()?;

    wait_for_key_async(stdin, Key::Char('p'))?;

    'outer: loop {
        let result = main.run(stdout, stdin)?;

        if let Some(score) = result {
            let mut scores = main.high_scores()?;

            let added = scores.add(score);

            scores.save()?;

            print_scores(stdout, scores, added.map(|score| score.time()))?;

            write!(stdout,
                   "{}Game over! \n\rScore: {}\n\r\n\rPress {}p{} to play again, {}Esc{} exit to return to menu.",
                   termion::cursor::Goto(1, 15),
                   score,
                   color::Fg(color::LightWhite),
                   termion::style::Reset,
                   color::Fg(color::LightWhite),
                   termion::style::Reset)?;

            stdout.flush()?;

            'wait_for_key: loop {
                if let Some(key_or_error) = stdin.keys().next() {
                    let key = key_or_error?;

                    if let Key::Esc = key {
                        break 'outer;
                    } else if let Char('p') = key {
                        break 'wait_for_key;
                    }
                }

                thread::sleep(Duration::from_millis(10));
            }
        } else {
            break 'outer;
        }
    }

    Ok(())
}

fn print_scores<W: Write>(stdout: &mut W, scores: HighScores, highlight: Option<DateTime<Local>>) -> io::Result<()> {
    write!(stdout,
           "{}{}{}{}High scores{}",
           termion::clear::All,
           termion::cursor::Goto(10, 1),
           color::Fg(color::LightWhite),
           color::Bg(color::Green),
           termion::style::Reset)?;

    let mut y = 3;
    for score in scores.entries().iter() {
        if Some(score.time()) == highlight {
            write!(stdout, "{}",
                   color::Fg(color::Green))?;
        }
        write!(stdout,
               "{}{}{}",
               termion::cursor::Goto(10, y),
               score.score(),
               termion::style::Reset)?;
        y += 1;
    }
    Ok(())
}
