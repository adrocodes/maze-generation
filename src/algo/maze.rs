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
    fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            walls: [true, true, true, true],
        }
    }

    pub fn to_visited_id(&self) -> String {
        format!("{}x{}", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub cols: usize,
    pub rows: usize,
    pub matrix: Vec<Vec<Cell>>,
}

impl Grid {
    fn build_matrix(rows: usize, cols: usize) -> Vec<Vec<Cell>> {
        let mut matrix = Vec::<Vec<Cell>>::with_capacity(rows);
        for y in 0..rows {
            let mut row = Vec::<Cell>::with_capacity(cols);

            for x in 0..cols {
                row.push(Cell::new(x, y));
            }

            matrix.push(row);
        }

        matrix
    }

    pub fn get_neighbor_cell(&self, starting_cell: &Cell, dir: Direction) -> Option<Cell> {
        // (row, col)
        let index_tuple: (usize, usize) = match dir {
            Direction::Top => {
                if starting_cell.y == 0 {
                    return None;
                }

                (starting_cell.y - 1, starting_cell.x)
            }
            Direction::Right => {
                if starting_cell.x == self.cols - 1 {
                    return None;
                }

                (starting_cell.y, starting_cell.x + 1)
            }
            Direction::Bottom => {
                if starting_cell.y == self.rows - 1 {
                    return None;
                }

                (starting_cell.y + 1, starting_cell.x)
            }
            Direction::Left => {
                if starting_cell.x == 0 {
                    return None;
                }

                (starting_cell.y, starting_cell.x - 1)
            }
        };

        Some(self.matrix[index_tuple.0][index_tuple.1])
    }

    pub fn new(rows: usize, cols: usize) -> Self {
        let matrix = Grid::build_matrix(rows, cols);

        Self { rows, cols, matrix }
    }
}
