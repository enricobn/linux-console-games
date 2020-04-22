extern crate rand;
extern crate termion;

use std::io::{stdin, stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::console::{cursor_up, reset};
use crate::console::Color::{self, Black, Blue, Cyan, DefaultColor, Green, Magenta, Red, White, Yellow};
use crate::grid::Grid;
use crate::shape::Shape;
use crate::tetris::Tetris;

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

    let mut tetris = Tetris::new(10, 10);

    goto(&mut stdout, 1, 2);

    tetris.print(&mut stdout);

    let stdin = stdin();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('n') => {
                tetris = tetris.next();

                goto(&mut stdout, 1, 2);
                tetris.print(&mut stdout)
            }
            Key::Char('q') => break,
            _ => {}
        }
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
