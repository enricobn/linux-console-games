use crate::arkanoid::arkanoid::Arkanoid;
use std::io::{Write, Error};
use std::{io, thread};
use termion::event::Key::Char;
use termion::event::Key;
use termion::async_stdin;
use termion::input::TermRead;
use std::time::Duration;
use crate::common::printutils::print_border;
use std::marker::PhantomData;
use crate::Main;
use crate::common::persistence::HighScores;

const WIDTH: u8 = 40;
const HEIGHT: u8 = 20;

pub struct ArkanoidMain<W: Write> {
    _marker: PhantomData<W>,
}

impl <W: Write> ArkanoidMain<W> {
    pub fn new() -> ArkanoidMain<W> {
        ArkanoidMain { _marker: PhantomData }
    }
}

impl <W: Write> Main<W> for ArkanoidMain<W> {
    fn name(&self) -> &'static str {
        "Arkanoid"
    }

    fn run(&self, stdout: &mut W) -> io::Result<Option<u32>> {
        let mut stdin = async_stdin().keys();
        let mut arkanoid = Arkanoid::new(WIDTH, HEIGHT);
        let mut score: u32 = 0;

        'outer: loop {
            for _i in 0..20 {
                let mut key_pressed = false;

                if let Some(key_or_error) = stdin.next() {
                    let key = key_or_error?;

                    if let Char('q') = key {
                        return Ok(None);
                    } else if let Key::Left = key {
                        arkanoid = arkanoid.left();
                        key_pressed = true;
                    } else if let Key::Right = key {
                        arkanoid = arkanoid.right();
                        key_pressed = true;
                    }
                }

                if key_pressed {
                    while stdin.next().is_some() {}
                    print(stdout, &arkanoid)?;
                }

                if let Some(ark) = arkanoid.next(0.05) {
                    arkanoid = ark;
                    print(stdout, &arkanoid)?;
                } else {
                    break 'outer
                }

                thread::sleep(Duration::from_millis(5));
            }
        }
        Result::Ok(Some(score))
    }

    fn high_scores(&self) -> Result<HighScores, Error> {
        HighScores::read(".arkanoid")
    }
}

fn print<W: Write>(term: &mut W, arkanoid: &Arkanoid) -> io::Result<()> {
    write!(term, "{}", termion::clear::All)?;
    print_border(term, 1, 1, WIDTH as u16 + 2, HEIGHT as u16 + 3)?;
    arkanoid.print(term, 1, 1)?;
    term.flush()
}
