#[derive(Clone, Debug)]
pub struct Point {
    pub x: i8,
    pub y: i8,
}

impl Point {
    pub fn new(x: i8, y: i8) -> Point {
        Point { x, y }
    }

    pub fn down(&self) -> Point {
        Point::new(self.x, self.y + 1)
    }

    pub fn right(&self) -> Point {
        Point::new(self.x + 1, self.y)
    }

    pub fn left(&self) -> Point {
        Point::new(self.x - 1, self.y)
    }

    pub fn up(&self) -> Point {
        Point::new(self.x, self.y - 1)
    }

    pub fn mv(&self, direction: &Direction) -> Point {
        match direction {
            Direction::North => self.up(),
            Direction::South => self.down(),
            Direction::East => self.right(),
            Direction::West => self.left()
        }
    }
}

#[derive(Clone, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}