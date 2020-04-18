extern crate rand;
mod console;
mod grid;
mod shape;
mod tetris;

use crate::console::Color::{self, Blue, White, Red, Yellow, Magenta, Cyan, Green, Black, DefaultColor};
use crate::console::{reset, cursor_up};
use crate::grid::Grid;
use crate::shape::Shape;
use crate::tetris::Tetris;

fn main() {

    let tetris = Tetris::new(10, 10);

    tetris.print();

    let tetris_next = tetris.next();

    tetris_next.print();

    let tetris_next1 = tetris_next.next();

    tetris_next1.print();

}
