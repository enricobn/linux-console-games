use std::io;
use termion::color;
use std::io::{Write, stdin};
use termion::event::Key;
use termion::input::TermRead;

pub fn choose<W: Write>(stdout: &mut W, menu: &Vec<&str>, x: u16, y: u16) -> io::Result<Option<u8>> {
    let mut index : u8 = 0;

    'outer: loop {

        for i in 0..menu.len() {
            write!(stdout,
                   "{}",
                   termion::cursor::Goto(x, y + i as u16))?;

            let menu_item = menu[i];

            if index == i as u8 {
                write!(stdout, "{} ", color::Bg(color::Cyan))?;
            } else {
                write!(stdout, "{} ", termion::style::Reset)?;
            }
            write!(stdout,
                   "{}{}",
                   menu_item,
                   termion::style::Reset)?;
        };

        stdout.flush()?;

        let stdin = stdin();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Up => {
                    index -= 1;
                    break;
                }
                Key::Down => {
                    index += 1;
                    if index >= menu.len() as u8 {
                        index = menu.len() as u8 - 1;
                    }
                    break;
                }
                Key::Esc => {
                    return Result::Ok(None);
                }
                Key::Char('\n') => break 'outer,
                _ => break
            };
        }
    }

    Result::Ok(Some(index))

}