use std::io;
use std::io::Write;

use crate::common::point::{Direction, Point};

pub struct Snake {
    direction: Direction,
    points: Vec<Point>,
}

impl Snake {
    pub fn new(x: i8, y: i8, direction: Direction) -> Snake {
        Snake { points: vec!(Point::new(x, y)), direction }
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
        Snake { points: self.points.clone(), direction }
    }

    pub fn next(&self, eat: bool) -> Snake {
        let last = self.points.last().unwrap();

        let point = last.mv(&self.direction);
        let mut points = self.points.clone();

        if !eat {
            points.remove(0);
        }

        points.push(point);

        Snake { points, direction: self.direction.clone() }
    }

    pub fn print<W: Write>(&self, term: &mut W, border: bool) -> io::Result<()> {
        for point in self.points.iter() {
            write!(term, "{}#", termion::cursor::Goto(point.x as u16, point.y as u16))?;
        }
        Result::Ok(())
    }
}