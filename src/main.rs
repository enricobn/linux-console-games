extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate termion;

use std::{io, thread};
use std::io::{stdout, Write};
use std::io::Read;
use std::time::Duration;

use termion::async_stdin;
use termion::raw::IntoRawMode;

use crate::persistence::HighScores;
use crate::tetris::Tetris;

mod consolecolor;
mod grid;
mod persistence;
mod shape;
mod tetris;

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

    attempt! {{
        run(&mut stdout);
    } catch(e) {
        write!(stdout,
               "{}\n\r",
               termion::cursor::Show)
            .unwrap();

        stdout.flush().unwrap();

        println!("Failed to run: {}", e);
    }}
}

fn run<W: Write>(mut stdout: &mut W) -> io::Result<()> {
    let mut scores = HighScores::read(".tetris")?;

    write!(stdout,
           "{}{}q to exit, left and right arrow to move{}down to rotate clockwise, up to rotate counterclockwise.\r\n{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Goto(1, 2),
           termion::cursor::Hide)?;

    stdout.flush()?;

    let mut tetris = Tetris::new(10, 20);

    let mut stdin = async_stdin().bytes();

    let mut score: u32 = 0;

    print(&mut stdout, &mut tetris, score)?;

    'outer: loop {
        for _i in 0..40 {
            let mut key_pressed = false;

            let b = stdin.next();
            if let Some(Ok(b'q')) = b {
                break 'outer;
            } else if let Some(Ok(b' ')) = b {
                let (packed, new_tetris) = tetris.fall()?;
                score += packed as u32 * 1000;
                tetris = new_tetris;
                key_pressed = true;
            } else if let Some(Ok(27)) = b {
                let b = stdin.next();
                if let Some(Ok(91)) = b {
                    let b = stdin.next();
                    if let Some(Ok(68)) = b {
                        tetris = tetris.left()?;
                        key_pressed = true;
                    } else if let Some(Ok(67)) = b {
                        tetris = tetris.right()?;
                        key_pressed = true;
                    } else if let Some(Ok(65)) = b {
                        tetris = tetris.rotate_left()?;
                        key_pressed = true;
                    } else if let Some(Ok(66)) = b {
                        tetris = tetris.rotate_right()?;
                        key_pressed = true;
                    }
                }
            }

            if key_pressed {
                while stdin.next().is_some() {}

                print(&mut stdout, &mut tetris, score)?;
            }

            thread::sleep(Duration::from_millis(10));
        }

        if let Ok(Some((packed, new_tetris))) = tetris.next() {
            tetris = new_tetris;

            score += packed as u32 * 1000;
            print(&mut stdout, &mut tetris, score)?;
        } else {
            // game as ended
            scores.add(score);
            scores.save()?;

            break 'outer;
        }
    }
    write!(stdout,
           "{}{}Game over! Score: {}\n\r",
           termion::clear::All,
           termion::cursor::Show,
           score)?;

    Result::Ok(())
}

fn print<W: Write>(mut stdout: &mut W, tetris: &mut Tetris, score: u32) -> io::Result<()> {
    write!(stdout,
           "{}Score: {}",
           termion::cursor::Goto(1, 3),
           score)?;
    clear_rec(stdout, 25, 5, 10, 5)?;
    tetris.print_next_shape(stdout, 30, 5)?;
    goto(&mut stdout, 1, 4)?;
    tetris.print(&mut stdout)
}

fn goto<W: Write>(stdout: &mut W, x: u16, y: u16) -> io::Result<()> {
    write!(stdout,
           "{}",
           termion::cursor::Goto(x, y))
}

pub fn clear_rec<W: Write>(stdout: &mut W, x: u8, y: u8, width: u8, height: u8) -> io::Result<()> {
    let row = " ".repeat(width as usize);
    write!(stdout, "{}", termion::style::Reset)?;
    for iy in y..(y + height) {
        goto(stdout, x as u16, iy as u16)?;
        write!(stdout, "{}", row)?;
    }
    Result::Ok(())
}
