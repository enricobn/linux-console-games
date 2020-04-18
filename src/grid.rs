use crate::console::{Color, reset};

pub struct Grid {
    width: u8,
    height: u8,
    cells: Vec<Vec<Color>>
}

impl Grid {

    pub fn new(width: u8, height: u8) -> Grid {
        let mut cells: Vec<Vec<Color>> = vec![];

        for _y in 0..height {
            let mut row: Vec<Color> = vec![];
            for _x in 0..width {
                row.push(Color::DefaultColor)
            }
            cells.push(row);
        }
        Grid {width, height, cells}
    }

    pub fn set(&self, x: u8, y: u8, color: Color) -> Grid {
        // TODO optimize: self.cells.to_vec() is a copy
        let new_cells = self.cells.to_vec().into_iter().enumerate()
            .map(|(iy, row)|
                row.into_iter().enumerate()
                    .map(|(ix, v_color)|
                        if iy == y.into() && ix == x.into() { color } else { v_color }
                    ).collect::<Vec<_>>()
            ).collect::<Vec<_>>();
        Grid { width: self.width, height: self.height, cells: new_cells }
    }

    pub fn print(&self) {
        for row in &self.cells {
            for color in row {
                color.background();
                print!("  ");
            }
            reset();
            println!()
        }
    }
}