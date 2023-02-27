use super::maze::{Cell, Direction};
use image::{ImageBuffer, Luma};
use std::convert::Into;

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

    pub fn generate_as_image(self) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let image: ImageBuffer<Luma<u8>, Vec<u8>> = self.into();
        image
    }
}

impl Into<ImageBuffer<Luma<u8>, Vec<u8>>> for Grid {
    fn into(self) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let border_width: u32 = 1;
        let gap: u32 = 1;
        let size: u32 = 1;
        let cols = self.cols as u32;
        let rows = self.rows as u32;

        let image_width = (cols * size) + (border_width * 2) + ((cols - 1) * gap);
        let image_height = (rows * size) + (border_width * 2) + ((rows - 1) * gap);

        let mut img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(image_width, image_height);

        for (row, row_vec) in self.matrix.iter().enumerate() {
            for (col, cell) in row_vec.iter().enumerate() {
                let top_left_x =
                    cell.x as u32 + border_width + (gap * col as u32) + ((size - 1) * col as u32);
                let top_left_y =
                    cell.y as u32 + border_width + (gap * row as u32) + ((size - 1) * row as u32);

                for x in 0..size {
                    for y in 0..size {
                        let pixel = img.get_pixel_mut(top_left_x + x, top_left_y + y);
                        pixel.0 = [255u8];
                    }

                    let s = x;

                    for (wall_idx, wall_state) in cell.walls.iter().enumerate() {
                        if *wall_state {
                            continue;
                        }

                        match wall_idx {
                            // Top
                            0 => {
                                for g in 0..gap {
                                    let offset_y = 1 + g;
                                    let offset_x = s;

                                    let pixel = img.get_pixel_mut(
                                        top_left_x + offset_x,
                                        top_left_y - offset_y,
                                    );
                                    pixel.0 = [255u8];
                                }
                            }
                            // Right
                            1 => {
                                // FIXME: breaks when size is more than 1
                                for g in 0..gap {
                                    let offset_x = s + (g - s);
                                    let offset_y = s;

                                    let pixel = img.get_pixel_mut(
                                        top_left_x + offset_x + size,
                                        top_left_y + offset_y,
                                    );
                                    pixel.0 = [255u8];
                                }
                            }
                            // Bottom
                            2 => {
                                for g in 0..gap {
                                    let offset_y = g;
                                    let offset_x = s;

                                    let pixel = img.get_pixel_mut(
                                        top_left_x + offset_x,
                                        top_left_y + offset_y + size,
                                    );
                                    pixel.0 = [255u8];
                                }
                            }
                            // Left
                            3 => {
                                // FIXME: breaks when size is more than 1
                                for g in 0..gap {
                                    let offset_x = 1 + s + (g - s);
                                    let offset_y = s;

                                    let pixel = img.get_pixel_mut(
                                        top_left_x - offset_x,
                                        top_left_y + offset_y,
                                    );
                                    pixel.0 = [255u8];
                                }
                            }
                            _ => {
                                println!("Well this shouldn't happen");
                            }
                        };
                    }
                }
            }
        }

        img
    }
}
