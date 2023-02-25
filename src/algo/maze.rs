pub trait MazeGenerate {
    fn generate(self) -> Self;
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

    pub fn new(rows: usize, cols: usize) -> Self {
        let matrix = Grid::build_matrix(rows, cols);

        Self { rows, cols, matrix }
    }
}
