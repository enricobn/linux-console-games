use std::{io, thread};
use std::io::{Error, Write};
use std::marker::PhantomData;
use std::time::Duration;

use rand::Rng;
use termion::async_stdin;
use termion::event::Key;
use termion::input::TermRead;

use crate::common::persistence::HighScores;
use crate::common::point::{Direction, Point};
use crate::Main;
use crate::snake::snake::Snake;

const FOOD: u8 = 10;
const WIDTH: u8 = 20;
const HEIGHT: u8 = 20;

pub struct SnakeMain<W: Write> {
    _marker: PhantomData<W>,
}

impl<W: Write> SnakeMain<W> {
    pub fn new() -> SnakeMain<W> {
        SnakeMain { _marker: PhantomData }
    }
}

impl<W: Write> Main<W> for SnakeMain<W> {
    fn name(&self) -> &'static str {
        "Snake"
    }

    fn run(&self, mut stdout: &mut W) -> io::Result<Option<u32>> {
        let mut snake = Snake::new(WIDTH, HEIGHT, Direction::East);
        let mut score: u32 = 0;
        let mut stdin = async_stdin().keys();
        let mut food: Vec<Point> = vec!();
        let mut rng = rand::thread_rng();

        for _i in 0..FOOD {
            food.push(Point::new(rng.gen_range(0, WIDTH) as i8,
                                 rng.gen_range(0, HEIGHT) as i8));
        }

        write!(stdout,
               "{}",
               termion::cursor::Hide)?;
        stdout.flush()?;

        let mut result : io::Result<Option<u32>> = Result::Ok(None);

        'outer: loop {
            for _i in 0..20 {
                let mut key_pressed = false;

                if let Some(key_or_error) = stdin.next() {
                    let key = key_or_error?;

                    if let Key::Esc = key {
                        break 'outer;
                    } else if let Key::Left = key {
                        snake = snake.mv(Direction::West);
                        key_pressed = true;
                    } else if let Key::Right = key {
                        snake = snake.mv(Direction::East);
                        key_pressed = true;
                    } else if let Key::Up = key {
                        snake = snake.mv(Direction::North);
                        key_pressed = true;
                    } else if let Key::Down = key {
                        snake = snake.mv(Direction::South);
                        key_pressed = true;
                    }
                }

                if key_pressed {
                    print(&mut stdout, &snake, &food, score)?;
                }

                thread::sleep(Duration::from_millis(10));
            }

            if let Some(next_snake) = snake.next(false) {
                let food_found = food.iter().enumerate()
                    .find(|(_i, point)| point.x == next_snake.last().x && point.y == next_snake.last().y)
                    .map(|(i, _point)| i);

                if let Some(food_index) = food_found {
                    food.remove(food_index);
                    food.push(Point::new(rng.gen_range(0, WIDTH) as i8,
                                         rng.gen_range(0, HEIGHT) as i8));
                    snake = snake.next(true).unwrap();
                    score += 10;
                } else {
                    snake = next_snake;
                }

                print(&mut stdout, &snake, &food, score)?;
            } else {
                result = Result::Ok(Some(score));
                break 'outer;
            }
        }

        while stdin.next().is_some() { }

        result
    }

    fn high_scores(&self) -> Result<HighScores, Error> {
        HighScores::read(".snake")
    }
}

fn print<W: Write>(mut stdout: &mut W, snake: &Snake, food: &Vec<Point>, score: u32) -> io::Result<()> {
    write!(stdout,
           "{}{}Score: {}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           score)?;

    for point in food.iter() {
        write!(stdout, "{}.", termion::cursor::Goto(point.x as u16 + 2, point.y as u16 + 3))?;
    }

    snake.print(&mut stdout, 1, 2)?;

    stdout.flush()
}