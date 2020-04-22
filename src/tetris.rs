use rand::prelude::*;

use crate::grid::Grid;
use crate::shape::{Point, Shape};
use std::io::{Stdout, Write};
use termion::raw::RawTerminal;

const STATE_INIT: u8 = 0;
const STATE_NORMAL: u8 = 1;
const STATE_NEW_PIECE: u8 = 2;

#[derive(Clone)]
pub struct Piece {
    shape: Shape,
    position: Point,
}

impl Piece {
    pub fn print(&self, grid: Grid) -> Grid {
        self.shape.print(grid, self.position.x as u8, self.position.y as u8)
    }

    pub fn clear(&self, grid: Grid) -> Grid {
        self.shape.clear(grid, self.position.x as u8, self.position.y as u8)
    }

    pub fn down(&self) -> Piece {
        Piece { shape: self.shape.clone(), position: self.position.down() }
    }
}

pub struct Tetris {
    state: u8,
    grid: Grid,
    current_piece: Piece,
    next_shape: Shape,
}

impl Tetris {
    fn random_shape() -> Shape {
        let shapes: Vec<Shape> = vec![Shape::l(), Shape::z(), Shape::s(), Shape::i(), Shape::o()];
        let mut rng = rand::thread_rng();
        shapes[rng.gen_range(0, &shapes.len())].clone()
    }

    pub fn new(width: u8, height: u8) -> Tetris {
        let current_piece = Piece { shape: Tetris::random_shape(), position: Point::new(width as i8 / 2, 1) };
        Tetris { state: STATE_INIT, grid: Grid::new(width, height), current_piece, next_shape: Tetris::random_shape() }
    }

    pub fn next(&self) -> Tetris {
        if self.state == STATE_INIT {
            Tetris {
                state: STATE_NORMAL,
                current_piece: self.current_piece.clone(),
                grid: self.current_piece.print(self.grid.clone()),
                next_shape: self.next_shape.clone(),
            }
        } else if self.state == STATE_NORMAL {
            // TODO collision
            let mut grid = self.current_piece.clear(self.grid.clone());
            let piece = self.current_piece.down();
            Tetris { state: STATE_NORMAL, current_piece: piece.clone(), grid: piece.print(grid), next_shape: self.next_shape.clone() }
        } else {
            // TODO
            let mut grid = self.current_piece.clear(self.grid.clone());
            let piece = self.current_piece.down();
            Tetris { state: STATE_NORMAL, current_piece: piece.clone(), grid: piece.print(grid), next_shape: self.next_shape.clone() }
        }
    }

    pub fn print<W: Write>(&self, term: &mut W) {
        self.grid.print(term, true)
    }
}