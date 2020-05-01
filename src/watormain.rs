use std::{io, thread};
use std::io::Read;
use std::io::Write;
use std::time::Duration;

use termion::async_stdin;

use crate::persistence::HighScores;
use crate::tetris::Tetris;
use crate::wator::Wator;

pub fn run<W: Write>(mut stdout: &mut W) -> io::Result<()> {
    write!(stdout,
           "{}{}{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Goto(1, 2),
           termion::cursor::Hide)?;

    stdout.flush()?;

    let mut wator = Wator::new(10, 10);

    let mut stdin = async_stdin().bytes();

    let mut score: u32 = 0;

    loop {
        print(&mut stdout, &mut wator)?;

        let b = stdin.next();
        if let Some(Ok(b'q')) = b {
            break;
        }
        thread::sleep(Duration::from_millis(1000));
        wator = wator.next();
    }

    Result::Ok(())
}

fn print<W: Write>(mut stdout: &mut W, wator: &Wator) -> io::Result<()> {
    write!(stdout,
           "{}",
           termion::cursor::Goto(1, 1))?;
    wator.print(&mut stdout, false)
}