use std::{io, thread};
use std::io::{Error, Read, Write};
use std::marker::PhantomData;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;

use crate::common::persistence::HighScores;
use crate::common::point::Direction;
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

impl<W: Write, R: Read> Main<W, R> for SnakeMain<W> {
    fn name(&self) -> &'static str {
        "Snake"
    }

    fn run(&self, mut stdout: &mut W, stdin: &mut R) -> io::Result<Option<u32>> {
        let mut snake = Snake::new(WIDTH, HEIGHT, Direction::East, FOOD);

        let mut result: io::Result<Option<u32>> = Result::Ok(None);

        'outer: loop {
            for _i in 0..20 {
                let mut key_pressed = false;

                if let Some(key_or_error) = stdin.keys().next() {
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
                    print(&mut stdout, &snake)?;
                }

                thread::sleep(Duration::from_millis(10));
            }

            if let Some(next_snake) = snake.next() {
                snake = next_snake;
                print(&mut stdout, &snake)?;
            } else {
                result = Result::Ok(Some(snake.score()));
                break 'outer;
            }
        }

        while stdin.keys().next().is_some() {}

        result
    }

    fn high_scores(&self) -> Result<HighScores, Error> {
        HighScores::read(".snake")
    }
}

fn print<W: Write>(mut stdout: &mut W, snake: &Snake) -> io::Result<()> {
    write!(stdout,
           "{}{}Score: {}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           snake.score())?;

    snake.print(&mut stdout, 1, 2)?;

    stdout.flush()
}