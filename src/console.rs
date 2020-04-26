use std::fmt::{Formatter, Error};
use termion::color;

#[derive(Clone,Copy, Debug, Eq, PartialEq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    DefaultColor,
}

fn print_color(s: &str) {
    print!("\x1B[{}m", s)
}

fn print_csi(s: &str) {
    print!("\x1B[{}", s)
}

impl termion::color::Color for Color {

    fn write_fg(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Color::Black => { f.write_str(color::Black.fg_str()) }
            Color::Red => { f.write_str(color::Red.fg_str()) }
            Color::Green => { f.write_str(color::Green.fg_str()) }
            Color::Yellow => { f.write_str(color::Yellow.fg_str()) }
            Color::Blue => { f.write_str(color::Blue.fg_str()) }
            Color::Magenta => { f.write_str(color::Magenta.fg_str()) }
            Color::Cyan => { f.write_str(color::Cyan.fg_str()) }
            Color::White => { f.write_str(color::White.fg_str()) }
            Color::DefaultColor => { f.write_str(color::Reset.fg_str()) }
        }
    }

    fn write_bg(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Color::Black => { f.write_str(color::Black.bg_str()) }
            Color::Red => { f.write_str(color::Red.bg_str()) }
            Color::Green => { f.write_str(color::Green.bg_str()) }
            Color::Yellow => { f.write_str(color::Yellow.bg_str()) }
            Color::Blue => { f.write_str(color::Blue.bg_str()) }
            Color::Magenta => { f.write_str(color::Magenta.bg_str()) }
            Color::Cyan => { f.write_str(color::Cyan.bg_str()) }
            Color::White => { f.write_str(color::White.bg_str()) }
            Color::DefaultColor => { f.write_str(color::Reset.bg_str()) }
        }

    }
}