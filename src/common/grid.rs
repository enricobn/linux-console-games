use std::io::{Error, Write, ErrorKind};
use std::io;

use termion::color;

use crate::common::point::Point;
use crate::common::consolecolor::Color;

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

    pub fn print<W: Write>(&self, term: &mut W, border: bool) -> io::Result<()> {
        if border { self.print_border_row(term)?; }

        for row in &self.cells {
            if border { write!(term, "{} ", color::Bg(color::White))?; }

            for color in row {
                write!(term, "{}  ", color::Bg(*color))?;
            }
            if border { write!(term, "{} {}\n\r", color::Bg(color::White), termion::style::Reset)?; } else { write!(term, "{}\n\r", termion::style::Reset)?; }
        }

        if border { self.print_border_row(term)?; }

        term.flush()
    }

    pub fn any_occupied(&self, points: &Vec<Point>) -> io::Result<bool> {
        let error = points.iter().any(|point| point.x < 0 || point.x >= self.width as i8
            || point.y < 0 || point.y >= self.height as i8);

        if error {
            return Result::Err(Error::new(ErrorKind::Other, "Out of bounds."));
        }

        Result::Ok(points.iter().any(|point| {
            self.cells[point.y as usize][point.x as usize] != Color::DefaultColor
        }))
    }

    pub fn any_vertical_out(&self, points: &Vec<Point>) -> bool {
        points.iter().any(|point| {
            point.y >= self.height as i8 || point.y < 0
        })
    }

    pub fn any_out(&self, points: &Vec<Point>) -> bool {
        points.into_iter().any(|point| {
            point.x >= self.width as i8 || point.y >= self.height as i8 || point.x < 0 || point.y < 0
        })
    }

    pub fn pack(&self) -> (u8, Grid) {
        let mut new_cells = self.cells.to_vec().into_iter()
            .filter(|row|
                row.into_iter().filter(|color| **color == Color::DefaultColor).count() > 0)
            .collect::<Vec<_>>();
        let mut packed = 0;
        while new_cells.len() as u8 != self.height {
            packed += 1;
            new_cells.insert(0, Grid::create_empty_row(self.width))
        }
        (packed, Grid { width: self.width, height: self.height, cells: new_cells })
    }

    fn print_border_row<W: Write>(&self, term: &mut W) -> io::Result<()> {
        write!(term, "{} ", color::Bg(color::White))?;
        for _ in 0..self.width {
            write!(term, "  ")?;
        }
        write!(term, " {}\n\r", termion::style::Reset)
    }
}

