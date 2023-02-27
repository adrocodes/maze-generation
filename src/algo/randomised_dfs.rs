use rand::seq::SliceRandom;
use std::collections::{HashSet, VecDeque};

use super::maze::{Cell, Direction, Grid, MazeGenerate};

/// Based off this description and implementation description:
/// [Wikipedia](https://en.wikipedia.org/wiki/Maze_generation_algorithm#Randomized_depth-first_search)
/// Using this "Iterative implementation" with a Stack data structure
pub struct RandomisedDFS {
    pub grid: Grid,
}

impl RandomisedDFS {
    pub fn from_grid_size(rows: usize, cols: usize) -> Self {
        Self {
            grid: Grid::new(rows, cols),
            ..RandomisedDFS::default()
        }
    }

    fn get_non_visited_neighbor_cells(
        &self,
        cell: &Cell,
        visited: &HashSet<String>,
    ) -> Option<Vec<(Cell, Direction)>> {
        let grid = &self.grid;

        let cells = vec![
            (
                grid.get_neighbor_cell(&cell, Direction::Top),
                Direction::Top,
            ),
            (
                grid.get_neighbor_cell(&cell, Direction::Right),
                Direction::Right,
            ),
            (
                grid.get_neighbor_cell(&cell, Direction::Bottom),
                Direction::Bottom,
            ),
            (
                grid.get_neighbor_cell(&cell, Direction::Left),
                Direction::Left,
            ),
        ];

        let non_visited_cells = cells
            .into_iter()
            .filter_map(|item| {
                if let Some(cell) = item.0 {
                    Some((cell, item.1))
                } else {
                    None
                }
            })
            .filter(|c| {
                return !visited.contains(&c.0.to_visited_id());
            })
            .collect::<Vec<_>>();

        match non_visited_cells.len() {
            0 => None,
            _ => Some(non_visited_cells),
        }
    }

    fn update_cell_walls(&mut self, target_cell: Cell, collapse_at: Direction) {
        let updated_cell = &mut self.grid.matrix[target_cell.y][target_cell.x];

        match collapse_at {
            Direction::Top => {
                updated_cell.walls[0] = false;
            }
            Direction::Right => {
                updated_cell.walls[1] = false;
            }
            Direction::Bottom => {
                updated_cell.walls[2] = false;
            }
            Direction::Left => {
                updated_cell.walls[3] = false;
            }
        };
    }
}

impl Default for RandomisedDFS {
    fn default() -> Self {
        Self {
            grid: Grid::new(10, 10),
        }
    }
}

impl MazeGenerate for RandomisedDFS {
    fn generate(&mut self) {
        let mut stack = VecDeque::<Cell>::new();
        let mut visited = HashSet::<String>::new();
        let mut rng = rand::thread_rng();

        // Choose the initial cell, mark it as visited and push it to the stack
        let start_cell = self.grid.matrix[0][0];
        visited.insert(start_cell.to_visited_id());
        stack.push_back(start_cell);

        // While the stack is not empty
        while !stack.is_empty() {
            // While the stack is not empty
            if let Some(cell) = stack.pop_back() {
                // If the current cell has any neighbours which have not been visited
                if let Some(found_neighbors) = self.get_non_visited_neighbor_cells(&cell, &visited)
                {
                    let neighbor = found_neighbors.choose(&mut rng).unwrap();

                    // Remove the wall between the current cell and the chosen cell
                    self.update_cell_walls(cell, neighbor.1);
                    self.update_cell_walls(neighbor.0, neighbor.1.opposite());

                    // Push the current cell to the stack
                    stack.push_back(cell);

                    // Mark the chosen cell as visited and push it to the stack
                    visited.insert(neighbor.0.to_visited_id());
                    stack.push_back(neighbor.0);
                }
            }
        }
    }
}
