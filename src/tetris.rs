use crate::grid::Grid;
use crate::shape::{Shape, Point};
use rand::prelude::*;

//const STATE_INIT: u8 = 0;
const STATE_NORMAL: u8 = 1;
const STATE_NEW_PIECE: u8 = 2;

pub struct Piece {
    shape: Shape,
    position: Point
}

pub struct Tetris {
    state: u8,
    pub grid: Grid,
    current_piece: Piece,
    next_shape: Shape
}


impl Tetris {

    fn random_shape() -> Shape {
        let shapes: Vec<Shape> = vec![Shape::l(), Shape::z(), Shape::s(), Shape::i(), Shape::o()];
        let mut rng = rand::thread_rng();
        shapes[rng.gen_range(0, &shapes.len())].clone()
    }

    fn new(width: u8, height: u8) -> Tetris {
        let current_piece = Piece {shape: Tetris::random_shape(), position: Point::new(width as i8 / 2, 1) };
        Tetris {state: STATE_NORMAL, grid: Grid::new(width, height), current_piece, next_shape: Tetris::random_shape()}
    }
/*
    fn next(&self) -> Tetris {
        if self.state == STATE_INIT {

        } else if self.state == STATE_NORMAL {

        } else {

        }
    }
    */

}