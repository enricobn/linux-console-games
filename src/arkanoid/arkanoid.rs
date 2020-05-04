use std::f32::consts::PI;
use std::io;
use std::io::Write;

use termion::color;

use crate::common::consolecolor::Color;
use crate::common::point::Point;

const BAR_WIDTH: i8 = 5;
const BRICK_WIDTH: i8 = 4;

#[derive(Clone)]
struct Brick {
    pub position: Point,
    pub color: Color,
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

    pub fn collides(&self, point: &Point, width: u8) -> bool {
        self.y >= point.y as f32 && self.y < point.y as f32 + 1.0 && self.x >= point.x as f32 && self.x <= (point.x + width as i8) as f32
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
        let margin = 2 * BRICK_WIDTH;
        let bricks_count = (width - 2 * margin as u8) / BRICK_WIDTH as u8;
        let mut bricks: Vec<Brick> = Vec::new();

        for i in 0..bricks_count {
            bricks.push(Brick { position: Point::new(margin + i as i8 * BRICK_WIDTH, 5),
            color: Color::Red })
        }

        Arkanoid {
            width,
            height,
            ball: Ball { x: width as f32 / 2.0, y: height as f32 / 2.0, angle: PI / 4.0 },
            bar: Point::new(width as i8 / 2, height as i8 - 1),
            bricks,
        }
    }

    pub fn next(&self, delta: f32) -> Option<Arkanoid> {
        let mut ball = self.ball.next(delta);

        let mut field_rebound = false;

        if ball.collides(&self.bar, BAR_WIDTH as u8) {
            ball = Ball { x: self.ball.x, y: self.ball.y, angle: -self.ball.angle };
            ball = ball.next(delta);
            field_rebound = true;
        } else if ball.x < 0.0 || ball.x >= self.width as f32 {
            ball = Ball { x: self.ball.x, y: self.ball.y, angle: PI - self.ball.angle };
            ball = ball.next(delta);
            field_rebound = true;
        } else if ball.y < 0.0 {
            ball = Ball { x: self.ball.x, y: self.ball.y, angle: -self.ball.angle };
            ball = ball.next(delta);
            field_rebound = true;
        } else if ball.y >= self.bar.y as f32 {
            return None;
        }


        let mut bricks = self.bricks.clone();

        if !field_rebound {
            let brick_collisions: Vec<usize> =
                self.bricks.iter().enumerate()
                    .filter(|(_i, brick)| self.ball.collides(&brick.position, BRICK_WIDTH as u8) )
                    .map(|(i, _brick)| i)
                    .collect();

            for collision in brick_collisions.iter() {
                bricks.remove(*collision);
            }

            if !brick_collisions.is_empty() {
                ball = Ball { x: self.ball.x, y: self.ball.y, angle: -self.ball.angle };
                ball = ball.next(delta);
            }
        }

        Some(Arkanoid {
            width: self.width,
            height: self.height,
            ball,
            bar: self.bar.clone(),
            bricks,
        })
    }

    pub fn right(&self) -> Arkanoid {
        let point = self.bar.right();

        if point.x + BAR_WIDTH  > self.width  as i8{
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
        let bar_string = " ".repeat(BAR_WIDTH as usize);
        let brick_string = " ".repeat(BRICK_WIDTH as usize);

        for brick in self.bricks.iter() {
            write!(term, "{}{}{}",
                   color::Bg(brick.color),
                   termion::cursor::Goto(brick.position.x as u16 + x + 1, brick.position.y as u16 + y + 1),
                   brick_string)?;
        }

        write!(term, "{}{}{}",
               color::Bg(color::White),
               termion::cursor::Goto(self.bar.x as u16 + x + 1, self.bar.y as u16 + y + 1),
               bar_string)?;
        write!(term, "{}{}*",
               termion::style::Reset,
               termion::cursor::Goto(self.ball.x as u16 + x + 1, self.ball.y as u16 + y + 1))
    }
}