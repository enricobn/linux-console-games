use std::f32::consts::PI;

use crate::console::Color;
use crate::grid::Grid;

#[derive(Clone, Debug)]
pub struct Point {
    pub x: i8,
    pub y: i8,
}

impl Point {
    pub fn new(x: i8, y: i8) -> Point {
        Point { x, y }
    }

    pub fn down(&self) -> Point {
        Point::new(self.x, self.y + 1)
    }

    pub fn right(&self) -> Point {
        Point::new(self.x + 1, self.y)
    }

    pub fn left(&self) -> Point {
        Point::new(self.x - 1, self.y)
    }
}

#[derive(Clone)]
pub struct Shape {
    points: Vec<Point>,
    color: Color,
}

impl Shape {
    pub fn shapes() -> Vec<Shape> {
        vec!(Shape::o(), Shape::i(), Shape::l(), Shape::s(), Shape::z())
    }

    pub fn o() -> Shape {
        Shape {
            points: vec!(
                Point::new(-1, 0),
                Point::new(0, 0),
                Point::new(-1, 1),
                Point::new(0, 1)),
            color: Color::Yellow,
        }
    }

    pub fn i() -> Shape {
        Shape {
            points: vec!(
                Point::new(-2, 0),
                Point::new(-1, 0),
                Point::new(0, 0),
                Point::new(1, 0)),
            color: Color::Cyan,
        }
    }

    pub fn l() -> Shape {
        Shape {
            points: vec!(
                Point::new(-1, 0),
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(-1, 1)),
            color: Color::Blue,
        }
    }

    pub fn s() -> Shape {
        Shape {
            points: vec!(
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(-1, 1),
                Point::new(0, 1)),
            color: Color::Green,
        }
    }

    pub fn z() -> Shape {
        Shape {
            points: vec!(
                Point::new(-1, 0),
                Point::new(0, 0),
                Point::new(0, 1),
                Point::new(1, 1)),
            color: Color::Red,
        }
    }

    pub fn print(&self, grid: Grid, x: u8, y: u8) -> Grid {
        self.points.to_vec().into_iter()
            .fold(grid, |prev, point| prev.set((x as i8 + point.x) as u8, (y as i8 + point.y) as u8,
                                               self.color.clone()))
    }

    pub fn clear(&self, grid: Grid, x: u8, y: u8) -> Grid {
        self.points.to_vec().into_iter()
            .fold(grid.clone(), |prev, point| prev.set((x as i8 + point.x) as u8, (y as i8 + point.y) as u8,
                                                       Color::DefaultColor))
    }

    pub fn rotate_right(&self) -> Shape {
        self.rotate_by_angle(PI / 2.0)
    }

    pub fn rotate_left(&self) -> Shape {
        self.rotate_by_angle(-PI / 2.0)
    }

    pub fn to_points(&self, x: i8, y: i8) -> Vec<Point> {
        self.points.to_vec().into_iter()
            .map(|point| Point {x: x + point.x, y: y + point.y}).collect()
    }

    fn rotate_by_angle(&self, ang: f32) -> Shape {
        let points = self.points.to_vec().into_iter().map(|point| {
            let distance = ((point.x * point.x + point.y * point.y) as f32).sqrt();
            let angle = (point.y as f32).atan2(point.x as f32) + ang;
            Point { x: (distance * angle.cos()).round() as i8, y: (distance * angle.sin()).round() as i8 }
        }).collect::<Vec<_>>();

        Shape { points, color: self.color.clone() }
    }
}