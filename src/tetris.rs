use std::io::{Stdout, Write};

use rand::prelude::*;
use termion::raw::RawTerminal;

use crate::grid::Grid;
use crate::shape::{Point, Shape};
use std::borrow::Borrow;

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

    pub fn right(&self) -> Piece {
        Piece { shape: self.shape.clone(), position: self.position.right() }
    }

    pub fn left(&self) -> Piece {
        Piece { shape: self.shape.clone(), position: self.position.left() }
    }

    pub fn rotate_left(&self) -> Piece {
        Piece { shape: self.shape.rotate_left(), position: self.position.clone() }
    }

    pub fn rotate_right(&self) -> Piece {
        Piece { shape: self.shape.rotate_right(), position: self.position.clone() }
    }
}

#[derive(Clone)]
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

    pub fn next(&self) -> (u8, Tetris) {
        if self.state == STATE_INIT {
            (0, Tetris {
                state: STATE_NORMAL,
                current_piece: self.current_piece.clone(),
                grid: self.current_piece.print(self.grid.clone()),
                next_shape: self.next_shape.clone(),
            })
        } else if self.state == STATE_NORMAL {
            let grid = self.current_piece.clear(self.grid.clone());
            let piece = self.current_piece.down();
            let points = piece.shape.to_points(piece.position.x, piece.position.y);
            if grid.any_out(&points) || grid.any_occupied(&points) {
                let (packed, new_grid) = self.grid.pack();
                let (new_packed, tetris) = Tetris {
                    state: STATE_NEW_PIECE,
                    current_piece: piece.clone(),
                    grid: new_grid,
                    next_shape: self.next_shape.clone(),
                }.next();
                (packed + new_packed, tetris)
            } else {
                (0, Tetris {
                    state: STATE_NORMAL,
                    current_piece: piece.clone(),
                    grid: piece.print(grid),
                    next_shape: self.next_shape.clone(),
                })
            }
        } else {
            let current_piece = Piece {
                shape: self.next_shape.clone(),
                position: Point::new(self.grid.width as i8 / 2, 1),
            };
            let next_shape = Tetris::random_shape();
            (0, Tetris {
                state: STATE_NORMAL,
                current_piece: current_piece.clone(),
                grid: current_piece.print(self.grid.clone()),
                next_shape,
            })
        }
    }

    pub fn right(&self) -> Tetris {
        self.mv(|piece| piece.right())
    }

    pub fn left(&self) -> Tetris {
        self.mv(|piece| piece.left())
    }

    pub fn rotate_left(&self) -> Tetris {
        self.mv(|piece| piece.rotate_left())
    }

    pub fn rotate_right(&self) -> Tetris {
        self.mv(|piece| piece.rotate_right())
    }

    fn mv<F>(&self,f: F) -> Tetris where F: Fn(Piece) -> Piece {
        if self.state == STATE_NORMAL {
            let grid = self.current_piece.clear(self.grid.clone());
            let piece = f(self.current_piece.clone());
            let points = piece.shape.to_points(piece.position.x, piece.position.y);
            if grid.any_out(&points) || grid.any_occupied(&points) {
                (*self).clone()
            } else {
                Tetris {
                    state: STATE_NORMAL,
                    current_piece: piece.clone(),
                    grid: piece.print(grid),
                    next_shape: self.next_shape.clone(),
                }
            }
        } else {
            (*self).clone()
        }
    }

    pub fn fall(&self) -> (u8, Tetris) {
        let mut piece = self.current_piece.clone();
        let grid = piece.clear(self.grid.clone());
        loop {
            let piece_down = piece.down();
            let points = piece_down.shape.to_points(piece_down.position.x, piece_down.position.y);
            if grid.any_out(&points) || grid.any_occupied(&points) {
                let (packed, new_grid) = piece.print(grid).pack();
                return (packed, Tetris {
                    state: STATE_NEW_PIECE,
                    current_piece: piece.clone(),
                    grid: new_grid,
                    next_shape: self.next_shape.clone(),
                })
            }
            piece = piece_down;
        }
    }

    pub fn print<W: Write>(&self, term: &mut W) {
        self.grid.print(term, true)
    }
}