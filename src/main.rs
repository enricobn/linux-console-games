extern crate rand;
extern crate termion;

use std::io::{stdin, stdout, Write};
use std::io::Read;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::console::{cursor_up, reset};
use crate::console::Color::{self, Black, Blue, Cyan, DefaultColor, Green, Magenta, Red, White, Yellow};
use crate::grid::Grid;
use crate::shape::Shape;
use crate::tetris::Tetris;
use termion::async_stdin;
use std::thread;
use std::time::Duration;

mod console;
mod grid;
mod shape;
mod tetris;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout,
           "{}{}{}q to exit. Type stuff, use alt, and so on.\n\r",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide)
        .unwrap();
    stdout.flush().unwrap();

    let mut tetris = Tetris::new(10, 20);

    goto(&mut stdout, 1, 2);

    tetris.print(&mut stdout);

    let mut stdin = async_stdin().bytes();

    'outer: loop {
        for i in 0..80 {
            let mut key_pressed = false;

            let b = stdin.next();
            if let Some(Ok(b'q')) = b {
                break 'outer;
            } else if let Some(Ok(b' ')) = b {
                tetris = tetris.fall();
                key_pressed = true;
            } else if let Some(Ok(27)) = b {
                let b = stdin.next();
                if let Some(Ok(91)) = b {
                    let b = stdin.next();
                    if let Some(Ok(68)) = b {
                        tetris = tetris.left();
                        key_pressed = true;
                    } else if let Some(Ok(67)) = b {
                        tetris = tetris.right();
                        key_pressed = true;
                    } else if let Some(Ok(65)) = b {
                        tetris = tetris.rotate_left();
                        key_pressed = true;
                    } else if let Some(Ok(66)) = b {
                        tetris = tetris.rotate_right();
                        key_pressed = true;
                    }
                }
            }

            if key_pressed {
                while stdin.next().is_some() { }

                goto(&mut stdout, 1, 2);
                tetris.print(&mut stdout);
            }

            thread::sleep(Duration::from_millis(5));
        }

        tetris = tetris.next();

        goto(&mut stdout, 1, 2);
        tetris.print(&mut stdout);
    }

    write!(stdout,
           "{}",
           termion::cursor::Show)
        .unwrap();
}

fn goto<W: Write>(stdout: &mut W, x: u16, y: u16) {
    write!(stdout,
           "{}",
           termion::cursor::Goto(x, y))
        .unwrap();
}
