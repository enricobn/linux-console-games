mod console;
mod grid;
mod shape;

use crate::console::Color::{self, Blue, White, Red, Yellow, Magenta, Cyan, Green, Black, DefaultColor};
use crate::console::{reset, cursor_up};
use crate::grid::Grid;
use crate::shape::Shape;

fn main() {

    let grid = Grid::new(10, 10);

    let new_grid = Shape::l().rotate().print(grid, 3, 1);

    new_grid.print();

}
