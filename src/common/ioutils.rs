use std::{io, thread};
use std::io::{Read, Write};
use std::time::Duration;

use termion::color;
use termion::event::Key;
use termion::input::TermRead;

pub fn print_border<W: Write>(stdout: &mut W, x: u16, y: u16, width: u16, height: u16) -> io::Result<()> {
    write!(stdout, "{}", color::Bg(color::White))?;

    print_border_row(stdout, x, y, width)?;
    print_border_row(stdout, x, y + height as u16 - 1, width)?;

    for iy in 1..(height - 1) {
        write!(stdout, "{} ", termion::cursor::Goto(x, iy as u16 + y))?;
        write!(stdout, "{} ", termion::cursor::Goto(x + width as u16 - 1, iy as u16 + y))?;
    }

    write!(stdout, "{}", termion::style::Reset)?;

    Result::Ok(())
}

fn print_border_row<W: Write>(term: &mut W, x: u16, y: u16, width: u16) -> io::Result<()> {
    write!(term, "{}{}",
           termion::cursor::Goto(x, y),
           " ".repeat(width as usize))
}

pub fn wait_for_key_async<R: Read>(stdin: &mut R, key: Key) -> io::Result<()> {
    loop {
        if let Some(key_or_error) = stdin.keys().next() {
            let pressed_key = key_or_error?;

            if pressed_key == key {
                return Ok(());
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
}