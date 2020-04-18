mod console;
mod grid;

use crate::console::Color::{self, Blue, White, Red, Yellow, Magenta, Cyan, Green, Black, DefaultColor};
use crate::console::{reset, cursor_up};
use crate::grid::Grid;

fn main() {

    let grid = Grid::new(10, 10);

    let new_grid = grid.set(5, 5, Color::Magenta);

    new_grid.print();

}
