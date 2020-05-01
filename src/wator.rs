use std::borrow::Borrow;
use std::cell::Ref;
use std::collections::HashMap;
use std::io;
use std::io::Write;

use termion::color;

use crate::common::Point;
use rand::Rng;

trait Specie {
    fn mv(&self, north: Option<Box<dyn Specie>>,
          south: Option<Box<dyn Specie>>, east: Option<Box<dyn Specie>>,
          west: Option<Box<dyn Specie>>) -> MvResult;

    fn c(&self) -> char;

    fn box_clone(&self) -> Box<dyn Specie>;

    fn child(&self) -> Box<dyn Specie>;
}

impl Clone for Box<dyn Specie>
{
    fn clone(&self) -> Box<dyn Specie> {
        self.box_clone()
    }
}

#[derive(Clone, Debug)]
enum Movement {
    North,
    South,
    East,
    West,
}

struct MvResult {
    specie: Box<dyn Specie>,
    movement: Option<Movement>,
    child: bool,
}

#[derive(Clone)]
struct Fish {
    life: u16
}

#[derive(Clone)]
struct Shark {
    life: u16,
    energy: u16,
}

impl Fish {
    fn new() -> Fish {
        Fish { life: 0 }
    }
}

const FISH_REPRODUCTION_TIME : u16 = 100;

impl Specie for Fish {
    fn mv(&self, north: Option<Box<dyn Specie>>, south: Option<Box<dyn Specie>>,
          east: Option<Box<dyn Specie>>, west: Option<Box<dyn Specie>>) -> MvResult {
        let mut life = self.life + 1;

        let child = life > FISH_REPRODUCTION_TIME;

        if child {
            life = 0;
        }

        let mut possible_movements: Vec<Movement> = Vec::new();

        if let None = north {
            possible_movements.push(Movement::North)
        }

        if let None = south {
            possible_movements.push(Movement::South)
        }

        if let None = west {
            possible_movements.push(Movement::West)
        }

        if let None = east {
            possible_movements.push(Movement::East)
        }

        let movement = if possible_movements.is_empty() {
            None
        } else {
            let mut rng = rand::thread_rng();
            Some(possible_movements[rng.gen_range(0, possible_movements.len())].clone())
        };

        let me = Fish { life };
        MvResult { specie: Box::new(me), movement, child }
    }

    fn c(&self) -> char {
        '.'
    }

    fn box_clone(&self) -> Box<dyn Specie> {
        Box::new((*self).clone())
    }

    fn child(&self) -> Box<dyn Specie> {
        Box::new(Fish::new() )
    }
}

pub struct Wator {
    width: u8,
    height: u8,
    population: Vec<Vec<Option<Box<dyn Specie>>>>,
}

impl Wator {
    pub fn new(width: u8, height: u8) -> Wator {
        let mut population: Vec<Vec<Option<Box<dyn Specie>>>> = vec![];

        for _y in 0..height {
            let row = Wator::create_empty_row(width);
            population.push(row);
        }

        population[height as usize / 2][width as usize / 2] = Some(Box::new(Fish::new()));

        Wator { width, height, population }
    }

    pub fn next(&self) -> Wator {
        let mut population: Vec<Vec<Option<Box<dyn Specie>>>> = vec![];

        for _y in 0..self.height {
            let row = Wator::create_empty_row(self.width);
            population.push(row);
        }

        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                population[y][x] = self.population[y][x].clone();
            }
        }

        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                let north = self.safe_get(x as i8, y as i8 - 1, &population);
                let south = self.safe_get(x as i8, y as i8 + 1, &population);
                let east = self.safe_get(x as i8 + 1, y as i8, &population);
                let west = self.safe_get(x as i8 - 1, y as i8, &population);

                if let Some(specie) = &population[y][x].clone() {
                    population[y][x] = None;

                    let movement_result = specie.mv(north, south, east, west);

                    if let Some(mv) = movement_result.movement {
                        if movement_result.child {
                            population[y][x] = Some(specie.child());
                        }

                        match mv {
                            Movement::North => self.safe_put(x as i8, y as i8 - 1, &mut population,
                                                             movement_result.specie),
                            Movement::South => self.safe_put(x as i8, y as i8 + 1, &mut population,
                                                             movement_result.specie),
                            Movement::West => self.safe_put(x as i8 - 1, y as i8, &mut population,
                                                            movement_result.specie),
                            Movement::East => self.safe_put(x as i8 + 1, y as i8, &mut population,
                                                            movement_result.specie)
                        }
                    } else {
                        population[y][x] = Some(specie.clone());
                    }
                }
            }
        }

        Wator { width: self.width, height: self.height, population }
    }

    fn safe_get(&self, x: i8, y: i8, population: &Vec<Vec<Option<Box<dyn Specie>>>>) -> Option<Box<dyn Specie>> {
        let ix = if x < 0 {
            x + self.width as i8
        } else if x >= self.width as i8 {
            x - self.width as i8
        } else {
            x
        };

        let iy = if y < 0 {
            y + self.height as i8
        } else if y >= self.height as i8 {
            y - self.height as i8
        } else {
            y
        };

        if let Some(s) = population[iy as usize][ix as usize].as_ref() {
            Some(s.clone())
        } else {
            None
        }
    }

    fn safe_put(&self, x: i8, y: i8, population: &mut Vec<Vec<Option<Box<dyn Specie>>>>, specie: Box<dyn Specie>) {
        let ix = if x < 0 {
            x + self.width as i8
        } else if x >= self.width as i8 {
            x - self.width as i8
        } else {
            x
        };

        let iy = if y < 0 {
            y + self.height as i8
        } else if y >= self.height as i8 {
            y - self.height as i8
        } else {
            y
        };

        population[iy as usize][ix as usize] = Some(specie.clone());
    }

    pub fn print<W: Write>(&self, term: &mut W, border: bool) -> io::Result<()> {
        if border { self.print_border_row(term)?; }

        for row in &self.population {
            if border { write!(term, "{} ", color::Bg(color::White))?; }

            for s in row {
                if let Some(specie) = s {
                    write!(term, "{}", s.as_ref().unwrap().c())?;
                } else {
                    write!(term, " ")?;
                }
            }
            if border { write!(term, "{} {}\n\r", color::Bg(color::White), termion::style::Reset)?; } else { write!(term, "{}\n\r", termion::style::Reset)?; }
        }

        if border { self.print_border_row(term)?; }

        term.flush()
    }

    fn print_border_row<W: Write>(&self, term: &mut W) -> io::Result<()> {
        write!(term, "{} ", color::Bg(color::White))?;
        for _ in 0..self.width {
            write!(term, " ")?;
        }
        write!(term, " {}\n\r", termion::style::Reset)
    }

    fn create_empty_row(width: u8) -> Vec<Option<Box<dyn Specie>>> {
        let mut row: Vec<Option<Box<dyn Specie>>> = vec![];
        for _x in 0..width {
            row.push(None)
        }
        row
    }
}