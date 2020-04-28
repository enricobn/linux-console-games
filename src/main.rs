extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate termion;

use std::io::{stdout, Write};
use std::io::Read;
use std::thread;
use std::time::Duration;

use termion::async_stdin;
use termion::raw::IntoRawMode;

use crate::grid::Grid;
use crate::shape::Shape;
use crate::tetris::Tetris;
use crate::persistence::HighScores;


mod consolecolor;
mod grid;
mod persistence;
mod shape;
mod tetris;

fn main() {
    let mut scores = HighScores::read(".tetris").unwrap();

    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout,
           "{}{}q to exit, left and right arrow to move{}down to rotate clockwise, up to rotate counterclockwise.\r\n{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Goto(1, 2),
           termion::cursor::Hide)
        .unwrap();

    stdout.flush().unwrap();

    let mut tetris = Tetris::new(10, 20);

    let mut stdin = async_stdin().bytes();

    let mut score: u32 = 0;

    print(&mut stdout, &mut tetris, score);

    'outer: loop {
        for i in 0..40 {
            let mut key_pressed = false;

            let b = stdin.next();
            if let Some(Ok(b'q')) = b {
                break 'outer;
            } else if let Some(Ok(b' ')) = b {
                let (packed, new_tetris) = tetris.fall();
                score += packed as u32 * 1000;
                tetris = new_tetris;
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
                while stdin.next().is_some() {}

                print(&mut stdout, &mut tetris, score);
            }

            thread::sleep(Duration::from_millis(10));
        }

        if let Some((packed, new_tetris)) = tetris.next() {
            tetris = new_tetris;

            score += packed as u32 * 1000;
            print(&mut stdout, &mut tetris, score);
        } else {
            write!(stdout,
                   "{}Game over! Score: {}\n\r",
                   termion::clear::All,
                   score)
                .unwrap();
            scores.add(score);
            scores.save().unwrap();

            break 'outer
        }

    }

    write!(stdout,
           "{}",
           termion::cursor::Show)
        .unwrap();

    stdout.flush().unwrap();
}

fn print<W: Write>(mut stdout: &mut W, tetris: &mut Tetris, score: u32) {
    write!(stdout,
           "{}Score: {}",
           termion::cursor::Goto(1, 3),
           score)
        .unwrap();
    clear_rec(stdout, 25, 5, 10, 5);
    tetris.print_next_shape(stdout, 30, 5);
    goto(&mut stdout, 1, 4);
    tetris.print(&mut stdout);
}

fn goto<W: Write>(stdout: &mut W, x: u16, y: u16) {
    write!(stdout,
           "{}",
           termion::cursor::Goto(x, y))
        .unwrap();
}

pub fn clear_rec<W: Write>(stdout: &mut W, x: u8, y: u8, width: u8, height: u8) {
    let row = " ".repeat(width as usize);
    write!(stdout, "{}", termion::style::Reset);
    for iy in y..(y + height) {
        goto(stdout, x as u16, iy as u16);
        write!(stdout, "{}", row);
    }
}
