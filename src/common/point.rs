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
pub struct Pointf32 {
    pub x: f32,
    pub y: f32,
}

impl Pointf32 {
    pub fn new(x: f32, y: f32) -> Pointf32 {
        Pointf32 { x, y }
    }

    pub fn down(&self) -> Pointf32 {
        Pointf32::new(self.x, self.y + 1.0)
    }

    pub fn right(&self) -> Pointf32 {
        Pointf32::new(self.x + 1.0, self.y)
    }

    pub fn left(&self) -> Pointf32 {
        Pointf32::new(self.x - 1.0, self.y)
    }

    pub fn up(&self) -> Pointf32 {
        Pointf32::new(self.x, self.y - 1.0)
    }

    pub fn mv(&self, direction: &Direction) -> Pointf32 {
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