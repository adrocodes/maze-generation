mod algo;
mod graph;
mod util;

use std::collections::HashMap;

use algo::maze::MazeGenerate;
use graph::graph::Node;
use image::{GenericImageView, ImageBuffer, Luma};
use util::build_offset_getter;

use crate::{algo::maze::Direction, graph::builder::GraphBuilder};

const SOLUTION_PATH_COLOUR: [u8; 1] = [100u8];
const PATH_COLOUR: [u8; 1] = [255u8];
const MAZE_SIZE: (usize, usize) = (10, 10);
const STARTING_SPOT: (u32, u32) = (1, 0);
const ENDING_SPOT: (u32, u32) = ((MAZE_SIZE.0 as u32 * 2) - 1, MAZE_SIZE.1 as u32 * 2);

fn is_corridor(floors: [&bool; 4]) -> bool {
    let left_right = [false, true, false, true];
    let top_bottom = [true, false, true, false];
    let floors = floors.map(|b| *b);

    floors == left_right || floors == top_bottom
}

fn find_in_path(path: &Vec<Node<(u32, u32)>>, value: (u32, u32)) -> Option<&Node<(u32, u32)>> {
    path.iter().find(|n| n.value == value)
}

fn draw_solution(
    image: &mut ImageBuffer<Luma<u8>, Vec<u8>>,
    path: &Vec<Node<(u32, u32)>>,
    start: (u32, u32),
) {
    let mut next = find_in_path(path, start);

    while let Some(node) = next {
        let mut pixel = image.get_pixel_mut(node.value.0, node.value.1);
        pixel.0 = SOLUTION_PATH_COLOUR;

        match node.parent {
            Some(value) => {
                if value.0 != node.value.0 {
                    let x_range = value.0.min(node.value.0)..value.0.max(node.value.0);

                    for x in x_range {
                        let mut pixel = image.get_pixel_mut(x, node.value.1);
                        pixel.0 = SOLUTION_PATH_COLOUR;
                    }
                }

                if value.1 != node.value.1 {
                    let y_range = value.1.min(node.value.1)..value.1.max(node.value.1);

                    for y in y_range {
                        let mut pixel = image.get_pixel_mut(node.value.0, y);
                        pixel.0 = SOLUTION_PATH_COLOUR;
                    }
                }

                next = find_in_path(path, value);
            }
            None => next = None,
        };
    }
}

fn add_maze_start(image: &mut ImageBuffer<Luma<u8>, Vec<u8>>) {
    let mut pixel = image.get_pixel_mut(STARTING_SPOT.0, STARTING_SPOT.1);
    pixel.0 = PATH_COLOUR;
}

fn add_maze_end(image: &mut ImageBuffer<Luma<u8>, Vec<u8>>) {
    let mut pixel = image.get_pixel_mut(ENDING_SPOT.0, ENDING_SPOT.1);
    pixel.0 = PATH_COLOUR;
}

fn main() {
    let mut maze_algo = algo::RandomisedDFS::from_grid_size(MAZE_SIZE.0, MAZE_SIZE.1);
    maze_algo.generate();

    println!("Maze generated");

    let image = maze_algo.grid.generate_as_image();

    image.save("maze.png").unwrap();

    println!("Maze saved");

    let mut image = image::open("maze.png").unwrap();
    println!("Maze image open");

    let di = image.dimensions();
    let offset_getter = build_offset_getter((0, 0), (di.0, di.1));
    let image = image.as_mut_luma8().unwrap();
    let mut builder = GraphBuilder::<(u32, u32)>::new();

    add_maze_start(image);
    add_maze_end(image);

    let mut pixel_map = HashMap::<(u32, u32), bool>::new();
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if pixel.0 == PATH_COLOUR || pixel.0 == SOLUTION_PATH_COLOUR {
            pixel_map.insert((x, y), true);
            pixel.0 = PATH_COLOUR;
        }
    }

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if pixel.0 == PATH_COLOUR {
            let floors = [
                pixel_map
                    .get(&offset_getter(x, y, Direction::Top).unwrap_or((0, 0)))
                    .unwrap_or(&false),
                pixel_map
                    .get(&offset_getter(x, y, Direction::Right).unwrap_or((0, 0)))
                    .unwrap_or(&false),
                pixel_map
                    .get(&offset_getter(x, y, Direction::Bottom).unwrap_or((0, 0)))
                    .unwrap_or(&false),
                pixel_map
                    .get(&offset_getter(x, y, Direction::Left).unwrap_or((0, 0)))
                    .unwrap_or(&false),
            ];

            if !is_corridor(floors) {
                builder.add_node((x, y));
            }
        }
    }

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if pixel.0 == PATH_COLOUR {
            let floors = [
                pixel_map
                    .get(&offset_getter(x, y, Direction::Top).unwrap_or((0, 0)))
                    .unwrap_or(&false),
                pixel_map
                    .get(&offset_getter(x, y, Direction::Right).unwrap_or((0, 0)))
                    .unwrap_or(&false),
                pixel_map
                    .get(&offset_getter(x, y, Direction::Bottom).unwrap_or((0, 0)))
                    .unwrap_or(&false),
                pixel_map
                    .get(&offset_getter(x, y, Direction::Left).unwrap_or((0, 0)))
                    .unwrap_or(&false),
            ];

            if !is_corridor(floors) {
                let top = *floors[0];
                let left = *floors[3];

                if top {
                    let mut top_cell = offset_getter(x, y, Direction::Top);

                    while let Some(cell) = top_cell {
                        top_cell = offset_getter(cell.0, cell.1, Direction::Top);

                        if !pixel_map.get(&(cell.0, cell.1)).unwrap_or(&false) {
                            top_cell = None;
                        } else if builder.vertices.contains_key(&(cell.0, cell.1)) {
                            builder.add_edge((cell.0, cell.1), (x, y));
                            top_cell = None;
                        }
                    }
                }

                if left {
                    let mut left_cell = offset_getter(x, y, Direction::Left);

                    while let Some(cell) = left_cell {
                        left_cell = offset_getter(cell.0, cell.1, Direction::Left);

                        if !pixel_map.get(&(cell.0, cell.1)).unwrap_or(&false) {
                            left_cell = None;
                        } else if builder.vertices.contains_key(&(cell.0, cell.1)) {
                            builder.add_edge((cell.0, cell.1), (x, y));
                            left_cell = None;
                        }
                    }
                }
            }
        }
    }

    println!("Nodes generated");

    let graph = builder.build();

    println!("Has end: {:?}", graph.vertices.contains_key(&ENDING_SPOT));

    let path = &graph.bfs(STARTING_SPOT, ENDING_SPOT);

    if let Some(path) = path {
        println!("Path found - drawing solution");
        draw_solution(image, path, ENDING_SPOT);
    } else {
        println!("Path not found");
    }

    image.save("maze.png").unwrap();

    println!("Maze nodes saved");
}
