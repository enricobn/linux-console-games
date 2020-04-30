extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate termion;

use std::{io, thread};
use std::io::{stdin, stdout, Write};
use std::io::Read;
use std::time::Duration;

use termion::{async_stdin, color};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::persistence::HighScores;
use crate::tetris::Tetris;

mod consolecolor;
mod grid;
mod persistence;
mod shape;
mod tetris;
mod tetrismain;

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

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout,
           "{}{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide).unwrap();

    let mut index: i8 = 0;

    let menu = vec!("Tetris", "Snake", "Quit");

    'outer: loop {
        write!(stdout,
               "{}{}{}Console games{}\r\n\n",
               termion::clear::All,
               termion::cursor::Goto(1, 1),
               color::Bg(color::Red),
               termion::style::Reset).unwrap();

        for i in 0..menu.len() {
            let menu_item = menu[i];

            if i == menu.len() -1 {
                write!(stdout, "\n\r");
            }

            if index == i as i8 {
                write!(stdout, "{} ", color::Bg(color::Cyan)).unwrap();
            } else {
                write!(stdout, "{} ", termion::style::Reset).unwrap();
            }
            write!(stdout,
                   "{}\r\n{}",
                   menu_item,
                   termion::style::Reset).unwrap();
        };

        stdout.flush().unwrap();

        let stdin = stdin();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('q') => {
                    index = -1;
                    break 'outer;
                }
                Key::Up => {
                    index -= 1;
                    if index < 0 {
                        index = 0
                    }
                    break;
                }
                Key::Down => {
                    index += 1;
                    if index >= menu.len() as i8 {
                        index = menu.len() as i8 - 1;
                    }
                    break;
                }
                Key::Char('\n') => break 'outer,
                _ => break
            };
        }
    }

    if index < 0 {
        write!(stdout,
               "{}{}",
               termion::clear::All,
               termion::cursor::Show).unwrap();
        return;
    }

    attempt! {{
        run(&mut stdout, index);
    } catch(e) {
        write!(stdout,
               "{}\n\r",
               termion::cursor::Show)
            .unwrap();

        stdout.flush().unwrap();

        println!("Failed to run: {}", e);
    }}
}

fn run<W: Write>(mut stdout: &mut W, index: i8) -> io::Result<()> {
    if index == 0 {
        tetrismain::run(&mut stdout)
    } else if index == 1 {
        println!("Snake");
        Result::Ok(())
    } else {
        Result::Ok(())
    }
}
