use std::{io, thread};
use termion::{color, AsyncReader};
use std::io::Write;
use termion::event::Key;
use termion::input::Keys;
use std::time::Duration;

pub fn choose<W: Write>(stdout: &mut W, stdin: &mut Keys<AsyncReader>, menu: &Vec<&str>, x: u16, y: u16) -> io::Result<Option<u8>> {
    let mut index : i8 = 0;

    'outer: loop {

        for i in 0..menu.len() {
            write!(stdout,
                   "{}",
                   termion::cursor::Goto(x, y + i as u16))?;

            let menu_item = menu[i];

            if index == i as i8 {
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

        loop {
            if let Some(Ok(c)) = stdin.next() {
                match c {
                    Key::Up => {
                        index -= 1;
                        if index < 0 {
                            index = 0;
                        }
                        break;
                    }
                    Key::Down => {
                        index += 1;
                        if index >= menu.len() as i8 {
                            index = menu.len() as i8 - 1;
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

            thread::sleep(Duration::from_millis(50));
        }
    }

    Result::Ok(Some(index as u8))

}