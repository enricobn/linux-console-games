use std::{io, thread};
use std::io::{Error, Read, Write};
use std::marker::PhantomData;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;

use crate::arkanoid::arkanoid::Arkanoid;
use crate::common::ioutils::print_border;
use crate::common::persistence::HighScores;
use crate::Main;

const WIDTH: u8 = 40;
const HEIGHT: u8 = 20;

pub struct ArkanoidMain<W: Write> {
    _marker: PhantomData<W>,
}

impl<W: Write> ArkanoidMain<W> {
    pub fn new() -> ArkanoidMain<W> {
        ArkanoidMain { _marker: PhantomData }
    }
}

impl<W: Write, R: Read> Main<W, R> for ArkanoidMain<W> {
    fn name(&self) -> &'static str {
        "Arkanoid"
    }

    fn run(&self, stdout: &mut W, stdin: &mut R) -> io::Result<Option<u32>> {
        let mut arkanoid = Arkanoid::new(WIDTH, HEIGHT);

        let mut result: io::Result<Option<u32>> = Result::Ok(None);

        'outer: loop {
            for _i in 0..20 {
                let mut key_pressed = false;

                if let Some(key_or_error) = stdin.keys().next() {
                    let key = key_or_error?;

                    if let Key::Esc = key {
                        break 'outer;
                    } else if let Key::Left = key {
                        arkanoid = arkanoid.left();
                        key_pressed = true;
                    } else if let Key::Right = key {
                        arkanoid = arkanoid.right();
                        key_pressed = true;
                    }
                }

                if key_pressed {
                    print(stdout, &arkanoid)?;
                }

                if let Some(ark) = arkanoid.next(0.05) {
                    arkanoid = ark;
                    print(stdout, &arkanoid)?;
                } else {
                    result = Result::Ok(Some(arkanoid.score()));
                    break 'outer;
                }

                thread::sleep(Duration::from_millis(5));
            }
        }

        while stdin.keys().next().is_some() {}

        result
    }

    fn high_scores(&self) -> Result<HighScores, Error> {
        HighScores::read(".arkanoid")
    }
}

fn print<W: Write>(term: &mut W, arkanoid: &Arkanoid) -> io::Result<()> {
    write!(term, "{}{}Score: {}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           arkanoid.score()
    )?;
    print_border(term, 1, 2, WIDTH as u16 + 2, HEIGHT as u16 + 3)?;
    arkanoid.print(term, 1, 2)?;
    term.flush()
}
