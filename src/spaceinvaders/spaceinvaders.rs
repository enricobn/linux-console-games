use std::io;
use std::io::Write;

use rand::Rng;
use termion::color;

use crate::common::point::Pointf32;

const WIDTH: u8 = 40;
const HEIGHT: u8 = 20;
const MARGIN: u8 = 2;

pub struct SpaceInvaders {
    x: u8,
    enemies: Vec<Pointf32>,
    bullets: Vec<Pointf32>,
    enemy_bullets: Vec<Pointf32>,
    enemy_velocity: f32,
    score: u32,
}

impl SpaceInvaders {
    pub fn new() -> SpaceInvaders {
        let mut enemies: Vec<Pointf32> = Vec::new();
        for x in (3 * MARGIN..(WIDTH - 3 * MARGIN)).step_by(3) {
            for y in (0..8).step_by(2) {
                enemies.push(Pointf32::new(x as f32, y as f32));
            }
        }

        SpaceInvaders {
            x: WIDTH / 2,
            enemies,
            bullets: Vec::new(),
            enemy_bullets: Vec::new(),
            enemy_velocity: 0.1,
            score: 0,
        }
    }

    pub fn next(&self) -> Option<SpaceInvaders> {
        /*
                let nearest = bullets.iter().enumerate()
                    .min_by(|(i, point),(i1, point1)| (point1.x as u8 - self.x)
                        .cmp(&(point.x as u8 - self.x)));
        */

        let enemies: Vec<Pointf32> = self.enemies.iter()
            .filter(|enemy| self.bullets.iter()
                .all(|bullet| !SpaceInvaders::collides(bullet, enemy)))
            .map(|point| point.clone()).collect();

        let max_x_o = enemies.iter().map(|point| point.x as u8).max();
        let min_x_o = enemies.iter().map(|point| point.x as u8).min();
        let max_y_o = enemies.iter().map(|point| point.y as u8).max();

        if max_x_o.is_none() {
            return None;
        }

        let max_x = max_x_o.unwrap() as u8;
        let min_x = min_x_o.unwrap() as u8;
        let max_y = max_y_o.unwrap() as u8;

        let bullets: Vec<Pointf32> = self.bullets.iter()
            .filter(|bullet|
                self.enemies.iter()
                    .all(|enemy| !SpaceInvaders::collides(bullet, enemy)))
            .map(|point| point.up())
            .filter(|bullet| bullet.y >= 0.0).collect();

        let mut enemy_direction = self.enemy_velocity;

        let mut enemy_bullets: Vec<Pointf32> = self.enemy_bullets.iter().map(|point| point.down())
            .filter(|point| (point.y as u8) <= HEIGHT).collect();

        let enemies: Vec<Pointf32> = if max_x >= WIDTH - MARGIN || min_x <= MARGIN {
            enemy_direction = -enemy_direction;

            enemy_bullets.push(SpaceInvaders::enemy_fire(&enemies, max_y));

            enemies.iter().map(|point| Pointf32::new(point.x + enemy_direction, point.y + 1.0))
                .collect()
        } else {
            enemies.iter().map(|point| Pointf32::new(point.x + enemy_direction, point.y))
                .collect()
        };

        if enemies.iter().any(|enemy| enemy.y >= HEIGHT as f32) {
            return None;
        }

        let position = Pointf32::new(self.x as f32, HEIGHT as f32);

        if enemy_bullets.iter().any(|bullet| SpaceInvaders::collides(bullet, &position)) {
            return None;
        }

        let score = self.score + (1000 * (self.enemies.len() - enemies.len())) as u32;

        Some(SpaceInvaders {
            x: self.x,
            enemies,
            bullets,
            enemy_bullets,
            enemy_velocity: enemy_direction * 1.005,
            score,
        })
    }

    pub fn right(&self) -> SpaceInvaders {
        let mut x = self.x + 1;

        if x > WIDTH {
            x = self.x;
        }
        SpaceInvaders {
            x,
            enemies: self.enemies.clone(),
            bullets: self.bullets.clone(),
            enemy_bullets: self.enemy_bullets.clone(),
            enemy_velocity: self.enemy_velocity,
            score: self.score,
        }
    }

    pub fn left(&self) -> SpaceInvaders {
        let mut x = self.x as i8 - 1;
        if x < 0 {
            x = self.x as i8;
        }
        SpaceInvaders {
            x: x as u8,
            enemies: self.enemies.clone(),
            bullets: self.bullets.clone(),
            enemy_bullets: self.enemy_bullets.clone(),
            enemy_velocity: self.enemy_velocity,
            score: self.score,
        }
    }

    pub fn fire(&self) -> SpaceInvaders {
        let mut bullets = self.bullets.clone();

        bullets.push(Pointf32::new(self.x as f32, HEIGHT as f32 - 1.0));

        SpaceInvaders {
            x: self.x,
            enemies: self.enemies.clone(),
            bullets,
            enemy_bullets: self.enemy_bullets.clone(),
            enemy_velocity: self.enemy_velocity,
            score: self.score,
        }
    }

    pub fn print<W: Write>(&self, term: &mut W, x: u16, y: u16) -> io::Result<()> {
        write!(term, "{}",
               termion::style::Reset)?;

        for enemy in self.enemies.iter() {
            write!(term, "{}M",
                   termion::cursor::Goto(enemy.x as u16 + x + 1, enemy.y as u16 + y + 1))?;
        }

        for bullet in self.bullets.iter() {
            write!(term, "{}|",
                   termion::cursor::Goto(bullet.x as u16 + x + 1, bullet.y as u16 + y + 1))?;
        }

        write!(term, "{}",
               color::Fg(color::Red))?;

        for bullet in self.enemy_bullets.iter() {
            write!(term, "{}|",
                   termion::cursor::Goto(bullet.x as u16 + x + 1, bullet.y as u16 + y + 1))?;
        }

        write!(term, "{}",
               termion::style::Reset)?;

        write!(term, "{}A",
               termion::cursor::Goto(self.x as u16 + x + 1, HEIGHT as u16 + y + 1))
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    fn collides(bullet: &Pointf32, enemy: &Pointf32) -> bool {
        bullet.x as u16 == enemy.x as u16 && bullet.y as u16 == enemy.y as u16
    }

    fn enemy_fire(enemies: &Vec<Pointf32>, max_y: u8) -> Pointf32 {
        let enemies_to_fire: Vec<Pointf32> = enemies.iter().filter(|point| point.y as u8 == max_y)
            .map(|point| point.clone()).collect();

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0, enemies_to_fire.len());

        let bullet_x = enemies_to_fire[index].x;
        let bullet_y = enemies_to_fire[index].y + 2.0;

        Pointf32::new(bullet_x, bullet_y)
    }
}

