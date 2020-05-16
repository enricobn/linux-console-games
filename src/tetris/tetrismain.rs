use std::{io, thread};
use std::io::{Write, Error, Read};
use std::time::Duration;

use termion::event::Key;
use termion::event::Key::Char;
use termion::input::TermRead;

use crate::common::persistence::HighScores;
use crate::tetris::tetris::Tetris;
use crate::Main;
use std::marker::PhantomData;

pub struct TetrisMain<W: Write, R: Read> {
    _w_marker: PhantomData<W>,
    _r_marker: PhantomData<R>,
}

impl <W: Write, R: Read> TetrisMain<W, R> {
    pub fn new() -> TetrisMain<W, R> {
        TetrisMain { _w_marker: PhantomData, _r_marker: PhantomData }
    }
}

impl <W: Write, R: Read> Main<W,R> for TetrisMain<W,R> {

    fn name(&self) -> &'static str {
        "Tetris"
    }

    fn run(&self, mut stdout: &mut W, stdin: &mut R) -> io::Result<Option<u32>> {
        write!(stdout,
               "{}{}q to exit, left and right arrow to move{}down to rotate clockwise, up to rotate counterclockwise.\r\n",
               termion::clear::All,
               termion::cursor::Goto(1, 1),
               termion::cursor::Goto(1, 2))?;

        stdout.flush()?;

        let mut tetris = Tetris::new(10, 20);

        let mut score: u32 = 0;

        let mut result : io::Result<Option<u32>> = Result::Ok(None);

        print(&mut stdout, &mut tetris, score)?;

        'outer: loop {
            for _i in 0..40 {
                let mut key_pressed = false;

                if let Some(key_or_error) = stdin.keys().next() {
                    let key = key_or_error?;

                    if let Key::Esc = key {
                        break 'outer;
                    } else if let Char(' ') = key {
                        let (packed, new_tetris) = tetris.fall()?;
                        score += packed as u32 * 1000;
                        tetris = new_tetris;
                        key_pressed = true;
                    } else if let Key::Left = key {
                        tetris = tetris.left()?;
                        key_pressed = true;
                    } else if let Key::Right = key {
                        tetris = tetris.right()?;
                        key_pressed = true;
                    } else if let Key::Up = key {
                        tetris = tetris.rotate_left()?;
                        key_pressed = true;
                    } else if let Key::Down = key {
                        tetris = tetris.rotate_right()?;
                        key_pressed = true;
                    }
                }

                if key_pressed {
                    print(&mut stdout, &mut tetris, score)?;
                }

                thread::sleep(Duration::from_millis(10));
            }

            if let Ok(Some((packed, new_tetris))) = tetris.next() {
                tetris = new_tetris;

                score += packed as u32 * 1000;
                print(&mut stdout, &mut tetris, score)?;
            } else {
                result = Ok(Some(score));
                break 'outer;
            }
        }

        while stdin.keys().next().is_some() { }

        result
    }

    fn high_scores(&self) -> Result<HighScores, Error> {
        HighScores::read(".tetris")
    }
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
