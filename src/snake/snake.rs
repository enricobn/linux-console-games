use std::io;
use std::io::Write;

use rand::Rng;

use crate::common::ioutils::print_border;
use crate::common::point::{Direction, Point};

pub struct Snake {
    width: u8,
    height: u8,
    direction: Direction,
    points: Vec<Point>,
    food: Vec<Point>,
    score: u32,
}

impl Snake {
    pub fn new(width: u8, height: u8, direction: Direction, food_count: u8) -> Snake {
        let mut food = Vec::new();
        let mut rng = rand::thread_rng();

        for _i in 0..food_count {
            food.push(Point::new(rng.gen_range(0, width) as i8,
                                 rng.gen_range(0, height) as i8));
        }

        Snake { width, height, points: vec!(Point::new(width as i8 / 2, height as i8 / 2)), direction, food, score: 0 }
    }

    pub fn last(&self) -> &Point {
        self.points.last().unwrap()
    }

    pub fn mv(&self, direction: Direction) -> Snake {
        Snake {
            width: self.width,
            height: self.height,
            points: self.points.clone(),
            direction,
            food: self.food.clone(),
            score: self.score,
        }
    }

    pub fn next(&self) -> Option<Snake> {
        let last = self.points.last().unwrap();

        let point = last.mv(&self.direction);

        if point.x < 0 || point.x >= self.width as i8 ||
            point.y < 0 || point.y >= self.height as i8 ||
            self.points.iter().any(|p| p.x == point.x && p.y == point.y) {
            return None;
        }

        let mut food: Vec<Point> = self.food.clone();

        let food_found = self.food.iter().enumerate()
            .find(|(_i, point)| point.x == self.last().x && point.y == self.last().y)
            .map(|(i, _point)| i);

        let mut rng = rand::thread_rng();

        let mut points: Vec<Point> = self.points.clone();

        let mut score = self.score;

        if let Some(food_index) = food_found {
            food.remove(food_index);
            food.push(Point::new(rng.gen_range(0, self.width) as i8,
                                 rng.gen_range(0, self.height) as i8));
            score += 100;
        } else {
            points.remove(0);
        }

        points.push(point);

        Some(Snake {
            width: self.width,
            height: self.height,
            points,
            direction: self.direction.clone(),
            food,
            score,
        })
    }

    pub fn print<W: Write>(&self, term: &mut W, x: u16, y: u16) -> io::Result<()> {
        for point in self.food.iter() {
            write!(term, "{}.", termion::cursor::Goto(point.x as u16 + 2, point.y as u16 + 3))?;
        }

        for point in self.points.iter() {
            write!(term, "{}#", termion::cursor::Goto(point.x as u16 + x + 1, point.y as u16 + y + 1))?;
        }
        print_border(term, x, y, self.width as u16 + 2, self.height as u16 + 2)
    }

    pub fn score(&self) -> u32 {
        self.score
    }
}