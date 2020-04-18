use crate::console::Color;
use crate::grid::Grid;

#[derive(Copy, Clone)]
pub struct Point {
    x: i8,
    y: i8,
}

impl Point {
    pub fn new(x: i8, y: i8) -> Point {
        Point { x, y }
    }
}

pub struct Shape {
    points: Vec<Point>,
    color: Color
}

impl Shape {

    pub fn shapes() -> Vec<Shape> {
        vec!(Shape::o(), Shape::i(), Shape::l(), Shape::s(), Shape::z())
    }

    pub fn o() -> Shape {
        Shape { points: vec!(
            Point::new(-1, 0),
            Point::new(0, 0),
            Point::new(-1, 1),
            Point::new(0, 1)),
            color: Color::Yellow
        }
    }

    pub fn i() -> Shape {
        Shape { points: vec!(
            Point::new(-2, 0),
            Point::new(-1, 0),
            Point::new(0, 0),
            Point::new(1, 0)),
            color: Color::Cyan
        }
    }

    pub fn l() -> Shape {
        Shape { points: vec!(
            Point::new(-1, 0),
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(-1, 1)),
            color: Color::Blue
        }
    }

    pub fn s() -> Shape {
        Shape { points: vec!(
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(-1, 1),
            Point::new(0, 1)),
            color: Color::Green
        }
    }

    pub fn z() -> Shape {
        Shape { points: vec!(
            Point::new(-1, 0),
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(1, 1)),
            color: Color::Red
        }
    }

    pub fn print(&self, grid: Grid, x: u8, y: u8) -> Grid {
        self.points.to_vec().into_iter()
            .fold(grid, |prev, point| prev.set((x as i8 + point.x) as u8, (y as i8 + point.y) as u8, self.color))
    }

}