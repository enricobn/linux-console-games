use std::f32::consts::PI;
use std::io;
use std::io::Write;

use termion::color;

use crate::common::consolecolor::Color;
use crate::common::point::Point;

const BAR_WIDTH: i8 = 5;

#[derive(Clone)]
struct Brick {
    position: Point,
    color: Color,
}

#[derive(Clone)]
struct Ball {
    x: f32,
    y: f32,
    angle: f32,
}

impl Ball {
    pub fn next(&self, delta: f32) -> Ball {
        let x = self.x + self.angle.cos() * delta;
        let y = self.y + self.angle.sin() * delta;
        Ball { x, y, angle: self.angle }
    }
}

#[derive(Clone)]
pub struct Arkanoid {
    width: u8,
    height: u8,
    ball: Ball,
    bar: Point,
    bricks: Vec<Brick>,
}

impl Arkanoid {
    pub fn new(width: u8, height: u8) -> Arkanoid {
        Arkanoid {
            width,
            height,
            ball: Ball { x: width as f32 / 2.0, y: height as f32 / 2.0, angle: PI / 4.0 },
            bar: Point::new(width as i8 / 2, height as i8 - 1),
            bricks: vec!(),
        }
    }

    pub fn next(&self, delta: f32) -> Arkanoid {
        let mut ball = self.ball.next(delta);

        if ball.y >= self.bar.y as f32 && ball.x >= self.bar.x as f32 && ball.x <= (self.bar.x + BAR_WIDTH) as f32 {
            ball = Ball { x: ball.x, y: ball.y, angle: -self.ball.angle };
            ball = ball.next(delta)
        }

        Arkanoid {
            width: self.width,
            height: self.height,
            ball,
            bar: self.bar.clone(),
            bricks: self.bricks.clone(),
        }
    }

    pub fn right(&self) -> Arkanoid {
        let point = self.bar.right();

        if point.x >= self.width as i8 {
            self.clone()
        } else {
            Arkanoid {
                width: self.width,
                height: self.height,
                ball: self.ball.clone(),
                bar: point,
                bricks: self.bricks.clone(),
            }
        }
    }

    pub fn left(&self) -> Arkanoid {
        let point = self.bar.left();

        if point.x < 0 {
            self.clone()
        } else {
            Arkanoid {
                width: self.width,
                height: self.height,
                ball: self.ball.clone(),
                bar: point,
                bricks: self.bricks.clone(),
            }
        }
    }

    pub fn print<W: Write>(&self, term: &mut W, x: u16, y: u16) -> io::Result<()> {
        let bar = " ".repeat(BAR_WIDTH as usize);
        write!(term, "{}{}{}{}",
               termion::clear::All,
               color::Bg(color::White),
               termion::cursor::Goto(self.bar.x as u16 + x + 1, self.bar.y as u16 + y + 1),
               bar)?;
        write!(term, "{}{}*",
               termion::style::Reset,
               termion::cursor::Goto(self.ball.x as u16 + x + 1, self.ball.y as u16 + y + 1))
    }
}