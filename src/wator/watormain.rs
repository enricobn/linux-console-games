use std::{io, thread};
use std::io::Read;
use std::io::Write;
use std::time::Duration;

use termion::async_stdin;

use crate::wator::wator::Wator;

pub fn run<W: Write>(mut stdout: &mut W) -> io::Result<()> {
    write!(stdout,
           "{}{}{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Goto(1, 2),
           termion::cursor::Hide)?;

    stdout.flush()?;

    let mut wator = Wator::new(80, 40);

    let mut stdin = async_stdin().bytes();

    let mut time: u32 = 0;

    loop {
        time += 1;

        print(&mut stdout, &mut wator)?;

        let b = stdin.next();
        if let Some(Ok(b'q')) = b {
            break;
        }
        thread::sleep(Duration::from_millis(50));
        wator = wator.next();

        let (fishes, sharks) = wator.count();

        if fishes == 0 || sharks == 0 {
            break;
        }
    }

    write!(stdout,
           "{}Game over after {} cycles.\n\r",
           termion::clear::All,
           time)?;

    Result::Ok(())
}

fn print<W: Write>(mut stdout: &mut W, wator: &Wator) -> io::Result<()> {
    write!(stdout,
           "{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1))?;
    wator.print(&mut stdout, true)
}