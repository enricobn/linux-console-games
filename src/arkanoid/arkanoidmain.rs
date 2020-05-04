use crate::arkanoid::arkanoid::Arkanoid;
use std::io::Write;
use std::{io, thread};
use termion::event::Key::Char;
use termion::event::Key;
use termion::async_stdin;
use termion::input::TermRead;
use std::time::Duration;
use crate::common::printutils::print_border;

const WIDTH: u8 = 40;
const HEIGHT: u8 = 20;

pub fn run<W: Write>(stdout: &mut W) -> io::Result<()> {
    let mut stdin = async_stdin().keys();
    let mut arkanoid = Arkanoid::new(WIDTH, HEIGHT);

    'outer: loop {
        for _i in 0..20 {
            let mut key_pressed = false;

            if let Some(key_or_error) = stdin.next() {
                let key = key_or_error?;

                if let Char('q') = key {
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
    Result::Ok(())

}

fn print<W: Write>(term: &mut W, arkanoid: &Arkanoid) -> io::Result<()> {
    write!(term, "{}", termion::clear::All)?;
    print_border(term, 1, 1, WIDTH as u16 + 2, HEIGHT as u16 + 3)?;
    arkanoid.print(term, 1, 1)?;
    term.flush()
}
