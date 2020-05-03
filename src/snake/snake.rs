use std::io;
use std::io::Write;

use crate::common::point::{Direction, Point};
use crate::common::printutils::print_border;

pub struct Snake {
    width: u8,
    height: u8,
    direction: Direction,
    points: Vec<Point>,
}

impl Snake {
    pub fn new(width: u8, height: u8, direction: Direction) -> Snake {
        Snake { width, height, points: vec!(Point::new(width as i8 / 2, height as i8 / 2)), direction }
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn points(&self) -> &Vec<Point> {
        &self.points
    }

    pub fn last(&self) -> &Point {
        self.points.last().unwrap()
    }

    pub fn mv(&self, direction: Direction) -> Snake {
        Snake { width: self.width, height: self.height, points: self.points.clone(), direction }
    }

    pub fn next(&self, eat: bool) -> Option<Snake> {
        let last = self.points.last().unwrap();

        let point = last.mv(&self.direction);

        if point.x < 0 || point.x >= self.width as i8 ||
            point.y < 0 || point.y >= self.height as i8 ||
            self.points.iter().any(|p| p.x == point.x && p.y == point.y) {
            return None;
        }

        let mut points = self.points.clone();

        if !eat {
            points.remove(0);
        }

        points.push(point);

        Some(Snake { width: self.width, height: self.height, points, direction: self.direction.clone() })
    }

    pub fn print<W: Write>(&self, term: &mut W, x: u16, y: u16) -> io::Result<()> {
        for point in self.points.iter() {
            write!(term, "{}#", termion::cursor::Goto(point.x as u16 + x + 1, point.y as u16 + y + 1))?;
        }
        print_border(term, x, y, self.width as u16 + 2, self.height as u16 + 2)
    }


}