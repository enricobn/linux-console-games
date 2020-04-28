use std::io::Write;

use termion::color;
use termion::raw::RawTerminal;

use crate::consolecolor::Color;
use crate::shape::Point;

#[derive(Clone)]
pub struct Grid {
    pub width: u8,
    pub height: u8,
    cells: Vec<Vec<Color>>,
}

impl Grid {
    pub fn new(width: u8, height: u8) -> Grid {
        let mut cells: Vec<Vec<Color>> = vec![];

        for _y in 0..height {
            let row = Grid::create_empty_row(width);
            cells.push(row);
        }
        Grid { width, height, cells }
    }

    fn create_empty_row(width: u8) -> Vec<Color> {
        let mut row: Vec<Color> = vec![];
        for _x in 0..width {
            row.push(Color::DefaultColor)
        }
        row
    }

    pub fn set(&self, x: u8, y: u8, color: Color) -> Grid {
        let new_cells = self.cells.iter().enumerate()
            .map(|(iy, row)|
                row.iter().enumerate()
                    .map(|(ix, v_color)|
                        if iy as u8 == y && ix as u8 == x { color.clone() } else { v_color.clone() }
                    ).collect::<Vec<_>>()
            ).collect::<Vec<_>>();
        Grid { width: self.width, height: self.height, cells: new_cells }
    }

    pub fn print<W: Write>(&self, term: &mut W, border: bool) {
        if border { self.print_row(term); }

        for row in &self.cells {
            if border {write!(term, "{} ", color::Bg(color::White)).unwrap(); }

            for color in row {
                write!(term, "{}  ", color::Bg(*color));
            }
            if border { write!(term, "{} {}\n\r", color::Bg(color::White), termion::style::Reset).unwrap(); }
            else { write!(term, "{}\n\r", termion::style::Reset).unwrap(); }
        }

        if border { self.print_row(term); }

        term.flush().unwrap();
    }

    pub fn any_occupied(&self, points: &Vec<Point>) -> bool {
        points.iter().any(|point| {
            self.cells[point.y as usize][point.x as usize] != Color::DefaultColor
        })
    }

    pub fn any_horizontal_out(&self, points: &Vec<Point>) -> bool {
        points.iter().any(|point| {
            point.x >= self.width as i8 || point.x < 0
        })
    }

    pub fn any_vertical_out(&self, points: &Vec<Point>) -> bool {
        points.iter().any(|point| {
            point.x >= self.width as i8 || point.x < 0
        })
    }

    pub fn any_out(&self, points: &Vec<Point>) -> bool {
        points.into_iter().any(|point| {
            point.x >= self.width as i8 || point.y >= self.height as i8 || point.x < 0 || point.y < 0
        })
    }

    pub fn pack(&self) -> (u8, Grid) {
        let mut new_cells = self.cells.to_vec().into_iter()
            .filter(|row |
                row.into_iter().filter(|color| **color == Color::DefaultColor).count() > 0)
            .collect::<Vec<_>>();
        let mut packed = 0;
        while new_cells.len() as u8 != self.height {
            packed += 1;
            new_cells.insert(0, Grid::create_empty_row(self.width))
        }
        (packed, Grid { width: self.width, height: self.height, cells: new_cells })
    }

    fn print_row<W: Write>(&self, term: &mut W) {
        write!(term, "{} ", color::Bg(color::White)).unwrap();
        for _ in 0..self.width {
            write!(term, "  ").unwrap();
        }
        write!(term, " {}\n\r", termion::style::Reset).unwrap();
    }
}