use rand::Rng;
use std::borrow::Borrow;
use std::cell::Ref;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use termion::color;
use crate::common::point::Direction;

trait Specie {
    fn mv(&self, north: Option<Box<dyn Specie>>,
          south: Option<Box<dyn Specie>>, east: Option<Box<dyn Specie>>,
          west: Option<Box<dyn Specie>>) -> MvResult;

    fn c(&self) -> char;

    fn box_clone(&self) -> Box<dyn Specie>;

    fn child(&self) -> Box<dyn Specie>;

    fn can_be_eaten(&self) -> bool;
}

impl Clone for Box<dyn Specie>
{
    fn clone(&self) -> Box<dyn Specie> {
        self.box_clone()
    }
}

struct MvResult {
    specie: Option<Box<dyn Specie>>,
    direction: Option<Direction>,
    child: bool,
}

#[derive(Clone)]
struct Fish {
    life: u16
}

impl Fish {
    fn new() -> Fish {
        Fish { life: 0 }
    }
}

const FISH_REPRODUCTION_TIME: u16 = 50;
const SHARK_REPRODUCTION_TIME: u16 = 100;
const SHARK_INITIAL_ENERGY: u16 = 100;
const ENERGY_GAIN_ON_EAT: u16 = 10;
const SHARKS: u16 = 10;
const FISHES: u16 = 100;

impl Specie for Fish {
    fn mv(&self, north: Option<Box<dyn Specie>>, south: Option<Box<dyn Specie>>,
          east: Option<Box<dyn Specie>>, west: Option<Box<dyn Specie>>) -> MvResult {
        let mut life = self.life + 1;

        let child = life > FISH_REPRODUCTION_TIME;

        if child {
            life = 0;
        }

        let mut possible_movements: Vec<Direction> = Vec::new();

        if let None = north {
            possible_movements.push(Direction::North)
        }

        if let None = south {
            possible_movements.push(Direction::South)
        }

        if let None = west {
            possible_movements.push(Direction::West)
        }

        if let None = east {
            possible_movements.push(Direction::East)
        }

        let movement = if possible_movements.is_empty() {
            None
        } else {
            let mut rng = rand::thread_rng();
            Some(possible_movements[rng.gen_range(0, possible_movements.len())].clone())
        };

        let me = Fish { life };
        MvResult { specie: Some(Box::new(me)), direction: movement, child }
    }

    fn c(&self) -> char {
        '.'
    }

    fn box_clone(&self) -> Box<dyn Specie> {
        Box::new((*self).clone())
    }

    fn child(&self) -> Box<dyn Specie> {
        Box::new(Fish::new())
    }

    fn can_be_eaten(&self) -> bool {
        true
    }
}

#[derive(Clone)]
struct Shark {
    life: u16,
    energy: u16,
}

impl Shark {
    fn new() -> Shark {
        Shark { life: 0, energy: SHARK_INITIAL_ENERGY }
    }
}

impl Specie for Shark {
    fn mv(&self, north: Option<Box<dyn Specie>>, south: Option<Box<dyn Specie>>,
          east: Option<Box<dyn Specie>>, west: Option<Box<dyn Specie>>) -> MvResult {
        let mut life = self.life + 1;

        let child = life > SHARK_REPRODUCTION_TIME;

        if child {
            life = 0;
        }

        let mut energy = self.energy - 1;

        if energy == 0 {
            return MvResult { specie: None, direction: None, child: false };
        }

        let mut possible_movements: Vec<Direction> = Vec::new();
        let mut possible_eats: Vec<Direction> = Vec::new();

        if let None = north {
            possible_movements.push(Direction::North)
        } else if let Some(s) = north {
            if s.can_be_eaten() {
                possible_eats.push(Direction::North)
            }
        }

        if let None = south {
            possible_movements.push(Direction::South)
        } else if let Some(s) = south {
            if s.can_be_eaten() {
                possible_eats.push(Direction::South)
            }
        }

        if let None = west {
            possible_movements.push(Direction::West)
        } else if let Some(s) = west {
            if s.can_be_eaten() {
                possible_eats.push(Direction::West)
            }
        }

        if let None = east {
            possible_movements.push(Direction::East)
        } else if let Some(s) = east {
            if s.can_be_eaten() {
                possible_eats.push(Direction::East)
            }
        }

        let mut movement = None;

        if possible_eats.is_empty() {
            if !possible_movements.is_empty() {
                let mut rng = rand::thread_rng();
                movement = Some(possible_movements[rng.gen_range(0, possible_movements.len())].clone())
            }
        } else {
            let mut rng = rand::thread_rng();
            energy += ENERGY_GAIN_ON_EAT;
            movement = Some(possible_eats[rng.gen_range(0, possible_eats.len())].clone())
        }

        let me = Shark { life, energy };

        MvResult { specie: Some(Box::new(me)), direction: movement, child }
    }

    fn c(&self) -> char {
        '#'
    }

    fn box_clone(&self) -> Box<dyn Specie> {
        Box::new((*self).clone())
    }

    fn child(&self) -> Box<dyn Specie> {
        Box::new(Shark::new())
    }

    fn can_be_eaten(&self) -> bool {
        false
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

        let mut rng = rand::thread_rng();

        let mut fishes = FISHES;

        while fishes > 0 {
            let x = rng.gen_range(0, width as usize);
            let y = rng.gen_range(0, height as usize);

            if population[y][x].is_none() {
                fishes -= 1;
                population[y][x] = Some(Box::new(Fish::new()));
            }
        }

        let mut sharks = SHARKS;

        while sharks > 0 {
            let x = rng.gen_range(0, width as usize);
            let y = rng.gen_range(0, height as usize);

            if population[y][x].is_none() {
                sharks -= 1;
                population[y][x] = Some(Box::new(Shark::new()));
            }
        }

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

                    if let Some(specie) = movement_result.specie {
                        if let Some(mv) = movement_result.direction {
                            if movement_result.child {
                                population[y][x] = Some(specie.child());
                            }

                            match mv {
                                Direction::North => self.safe_put(x as i8, y as i8 - 1, &mut population,
                                                                  specie),
                                Direction::South => self.safe_put(x as i8, y as i8 + 1, &mut population,
                                                                  specie),
                                Direction::West => self.safe_put(x as i8 - 1, y as i8, &mut population,
                                                                 specie),
                                Direction::East => self.safe_put(x as i8 + 1, y as i8, &mut population,
                                                                 specie)
                            }
                        } else {
                            population[y][x] = Some(specie.clone());
                        }
                    }
                }
            }
        }

        Wator { width: self.width, height: self.height, population }
    }

    pub fn count(&self) -> (u16, u16) {
        let mut fishes: u16 = 0;
        let mut sharks: u16 = 0;

        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                if let Some(specie) = self.population[y][x].clone() {
                    if specie.can_be_eaten() {
                        fishes += 1;
                    } else {
                        sharks += 1;
                    }
                }
            }
        }
        (fishes, sharks)
    }

    fn safe_get(&self, x: i8, y: i8, population: &Vec<Vec<Option<Box<dyn Specie>>>>) -> Option<Box<dyn Specie>> {
        let (ix, iy) = self.safe_position(x, y);

        if let Some(s) = population[iy as usize][ix as usize].as_ref() {
            Some(s.clone())
        } else {
            None
        }
    }

    fn safe_put(&self, x: i8, y: i8, population: &mut Vec<Vec<Option<Box<dyn Specie>>>>, specie: Box<dyn Specie>) {
        let (ix, iy) = self.safe_position(x, y);

        population[iy as usize][ix as usize] = Some(specie.clone());
    }

    fn safe_position(&self, x: i8, y: i8) -> (i8, i8) {
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
        (ix, iy)
    }

    pub fn print<W: Write>(&self, term: &mut W, border: bool) -> io::Result<()> {
        let (fishes, sharks) = self.count();
        write!(term, "Fishes: {}  Sharks: {}\n\r", fishes, sharks)?;

        if border { self.print_border_row(term)?; }

        for row in &self.population {
            if border { write!(term, "{} {}", color::Bg(color::White), termion::style::Reset)?; }

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