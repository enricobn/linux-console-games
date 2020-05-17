use std::io;
use std::io::Write;

use rand::prelude::*;
use termion::color;

use crate::common::grid::Grid;
use crate::common::point::Point;
use crate::tetris::shape::Shape;

const STATE_INIT: u8 = 0;
const STATE_NORMAL: u8 = 1;
const STATE_NEW_PIECE: u8 = 2;
const START_Y: i8 = 2;

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
    score: u32,
}

impl Tetris {
    fn random_shape() -> Shape {
        let shapes: Vec<Shape> = Shape::shapes();
        let mut rng = rand::thread_rng();
        shapes[rng.gen_range(0, &shapes.len())].clone()
    }

    pub fn new(width: u8, height: u8) -> Tetris {
        let current_piece = Piece { shape: Tetris::random_shape(), position: Point::new(width as i8 / 2, START_Y) };
        Tetris {
            state: STATE_INIT,
            grid: Grid::new(width, height),
            current_piece,
            next_shape: Tetris::random_shape(),
            score: 0,
        }
    }

    /// returns None if game ended
    pub fn next(&self) -> io::Result<Option<Tetris>> {
        if self.state == STATE_INIT {
            Result::Ok(Some(Tetris {
                state: STATE_NORMAL,
                current_piece: self.current_piece.clone(),
                grid: self.current_piece.print(self.grid.clone()),
                next_shape: self.next_shape.clone(),
                score: self.score,
            }))
        } else if self.state == STATE_NORMAL {
            let grid = self.current_piece.clear(self.grid.clone());
            let piece = self.current_piece.down();
            let points = piece.shape.to_points(piece.position.x, piece.position.y);
            if grid.any_vertical_out(&points) || grid.any_occupied(&points)? {
                let (packed, new_grid) = self.grid.pack();
                if let Some(tetris) = (Tetris {
                    state: STATE_NEW_PIECE,
                    current_piece: piece.clone(),
                    grid: new_grid,
                    next_shape: self.next_shape.clone(),
                    score: self.score + 1000 * packed as u32,
                }.next())? {
                    Result::Ok(Some(tetris))
                } else {
                    Result::Ok(None)
                }
            } else {
                Result::Ok(Some(Tetris {
                    state: STATE_NORMAL,
                    current_piece: piece.clone(),
                    grid: piece.print(grid),
                    next_shape: self.next_shape.clone(),
                    score: self.score,
                }))
            }
        } else {
            let current_piece = Piece {
                shape: self.next_shape.clone(),
                position: Point::new(self.grid.width as i8 / 2, START_Y),
            };

            let points = current_piece.shape.to_points(current_piece.position.x, current_piece.position.y);
            if self.grid.any_occupied(&points)? {
                Result::Ok(None)
            } else {
                let next_shape = Tetris::random_shape();
                Result::Ok(Some(Tetris {
                    state: STATE_NORMAL,
                    current_piece: current_piece.clone(),
                    grid: current_piece.print(self.grid.clone()),
                    next_shape,
                    score: self.score,
                }))
            }
        }
    }

    pub fn right(&self) -> io::Result<Tetris> {
        self.mv(|piece| piece.right())
    }

    pub fn left(&self) -> io::Result<Tetris> {
        self.mv(|piece| piece.left())
    }

    pub fn rotate_left(&self) -> io::Result<Tetris> {
        self.mv(|piece| piece.rotate_left())
    }

    pub fn rotate_right(&self) -> io::Result<Tetris> {
        self.mv(|piece| piece.rotate_right())
    }

    fn mv<F: Fn(Piece) -> Piece>(&self, f: F) -> io::Result<Tetris> {
        if self.state == STATE_NORMAL {
            let grid = self.current_piece.clear(self.grid.clone());
            let piece = f(self.current_piece.clone());
            let points = piece.shape.to_points(piece.position.x, piece.position.y);
            if grid.any_out(&points) || grid.any_occupied(&points)? {
                Result::Ok((*self).clone())
            } else {
                Result::Ok(Tetris {
                    state: STATE_NORMAL,
                    current_piece: piece.clone(),
                    grid: piece.print(grid),
                    next_shape: self.next_shape.clone(),
                    score: self.score,
                })
            }
        } else {
            Result::Ok((*self).clone())
        }
    }

    pub fn fall(&self) -> io::Result<Tetris> {
        let mut piece = self.current_piece.clone();
        let grid = piece.clear(self.grid.clone());
        loop {
            let piece_down = piece.down();
            let points = piece_down.shape.to_points(piece_down.position.x, piece_down.position.y);
            if grid.any_vertical_out(&points) || grid.any_occupied(&points)? {
                let (packed, new_grid) = piece.print(grid).pack();
                return Result::Ok(Tetris {
                    state: STATE_NEW_PIECE,
                    current_piece: piece.clone(),
                    grid: new_grid,
                    next_shape: self.next_shape.clone(),
                    score: self.score + 1000 * packed as u32,
                });
            }
            piece = piece_down;
        }
    }

    pub fn print<W: Write>(&self, term: &mut W) -> io::Result<()> {
        self.grid.print(term, true)
    }

    pub fn print_next_shape<W: Write>(&self, term: &mut W, x: u8, y: u8) -> io::Result<()> {
        let points = self.next_shape.to_points(0, 0);
        write!(term, "{}", color::Bg(self.next_shape.color))?;
        for point in points {
            write!(term, "{}  ", termion::cursor::Goto((x + point.x as u8 * 2) as u16, (y + point.y as u8) as u16))?;
        }
        Result::Ok(())
    }

    pub fn score(&self) -> u32 {
        self.score
    }
}