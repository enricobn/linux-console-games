use std::{io, thread};
use std::io::{Error, Write, Read};
use std::marker::PhantomData;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;

use crate::common::persistence::HighScores;
use crate::Main;
use crate::spaceinvaders::spaceinvaders::SpaceInvaders;

pub struct SpaceInvadersMain<W: Write> {
    _marker: PhantomData<W>,
}

impl<W: Write> SpaceInvadersMain<W> {
    pub fn new() -> SpaceInvadersMain<W> {
        SpaceInvadersMain { _marker: PhantomData }
    }
}

impl<W: Write, R: Read> Main<W, R> for SpaceInvadersMain<W> {
    fn name(&self) -> &'static str {
        "Space Invaders"
    }

    fn run(&self, mut stdout: &mut W, stdin: &mut R) -> io::Result<Option<u32>> {
        write!(stdout,
               "{}",
               termion::clear::All)?;
        stdout.flush()?;

        let mut spaceinvaders = SpaceInvaders::new();
        let mut score: u32 = 0;

        let mut result : io::Result<Option<u32>> = Result::Ok(None);

        'outer: loop {
            for _i in 0..20 {
                let mut key_pressed = false;

                if let Some(key_or_error) = stdin.keys().next() {
                    let key = key_or_error?;

                    if let Key::Esc = key {
                        break 'outer;
                    } else if let Key::Left = key {
                        spaceinvaders = spaceinvaders.left();
                        key_pressed = true;
                    } else if let Key::Right = key {
                        spaceinvaders = spaceinvaders.right();
                        key_pressed = true;
                    } else if let Key::Char(' ') = key {
                        spaceinvaders = spaceinvaders.fire();
                        key_pressed = true;
                    }
                }

                if key_pressed {
                    print(&mut stdout, &spaceinvaders, score)?;
                }

                thread::sleep(Duration::from_millis(5));
            }

            if let Some(next_spaceinvaders) = spaceinvaders.next() {
                spaceinvaders = next_spaceinvaders;

                print(&mut stdout, &spaceinvaders, score)?;
            } else {
                result = Result::Ok(Some(score));
                break 'outer;
            }
        }

        while stdin.keys().next().is_some() { }

        result
    }

    fn high_scores(&self) -> Result<HighScores, Error> {
        HighScores::read(".spaceinvaders")
    }
}

fn print<W: Write>(mut stdout: &mut W, spaceinvaders: &SpaceInvaders, score: u32) -> io::Result<()> {
    write!(stdout,
           "{}{}Score: {}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           score)?;

    spaceinvaders.print(&mut stdout, 1, 2)?;

    stdout.flush()
}