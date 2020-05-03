use crate::arkanoid::arkanoid::Arkanoid;
use std::io::Write;
use std::{io, thread};
use termion::event::Key::Char;
use termion::event::Key;
use termion::async_stdin;
use termion::input::TermRead;
use std::time::Duration;

const WIDTH: u8 = 40;
const HEIGHT: u8 = 20;

pub fn run<W: Write>(mut stdout: &mut W) -> io::Result<()> {
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
                arkanoid.print(stdout, 1, 1)?;
                stdout.flush()?;
            }

            arkanoid = arkanoid.next(0.05);
            arkanoid.print(stdout, 1, 1)?;
            stdout.flush()?;

            thread::sleep(Duration::from_millis(5));
        }


    }
    Result::Ok(())

}