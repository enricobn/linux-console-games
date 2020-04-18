#[derive(Copy, Clone)]
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

impl Color {

    pub fn foreground(&self) {
        match self {
            Color::Black => { print_color("30") }
            Color::Red => { print_color("31") }
            Color::Green => { print_color("32") }
            Color::Yellow => { print_color("33") }
            Color::Blue => { print_color("34") }
            Color::Magenta => { print_color("35") }
            Color::Cyan => { print_color("36") }
            Color::White => { print_color("37") }
            Color::DefaultColor => { print_color("39") }
        }
    }

    pub fn background(&self) {
        match self {
            Color::Black => { print_color("40") }
            Color::Red => { print_color("41") }
            Color::Green => { print_color("42") }
            Color::Yellow => { print_color("43") }
            Color::Blue => { print_color("44") }
            Color::Magenta => { print_color("45") }
            Color::Cyan => { print_color("46") }
            Color::White => { print_color("47") }
            Color::DefaultColor => { print_color("49") }
        }
    }

}

pub fn reset() {
    print_color("0")
}

pub fn cursor_up(n: i8) {
    print_csi("1A")
}
