extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate termion;

use std::io;
use std::io::{stdout, Write};

use termion::color;
use termion::raw::IntoRawMode;

mod consolecolor;
mod common;
mod grid;
mod menu;
mod persistence;
mod shape;
mod tetris;
mod tetrismain;
mod wator;
mod watormain;

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
           "{}{}{}{}Console games{}\r\n\n",
           termion::cursor::Hide,
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           color::Bg(color::Red),
           termion::style::Reset).unwrap();

    let menu = vec!("Tetris", "Wator");

    let choice = menu::choose(&mut stdout, &menu, 1, 3).unwrap();

    if let Some(index) = choice {
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

    write!(stdout,
           "{}{}\r\n",
           termion::clear::All,
           termion::cursor::Show).unwrap();
}

fn run<W: Write>(mut stdout: &mut W, index: u8) -> io::Result<()> {
    if index == 0 {
        tetrismain::run(&mut stdout)
    } else if index == 1 {
        watormain::run(&mut stdout)
    } else {
        Result::Ok(())
    }
}
