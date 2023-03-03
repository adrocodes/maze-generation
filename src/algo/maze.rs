pub trait MazeGenerate {
    fn generate(&mut self);
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Top => Direction::Bottom,
            Direction::Right => Direction::Left,
            Direction::Bottom => Direction::Top,
            Direction::Left => Direction::Right,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    /// [top, right, bottom, left]
    pub walls: [bool; 4],
}

impl Cell {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            walls: [true, true, true, true],
        }
    }

    pub fn to_visited_id(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}
