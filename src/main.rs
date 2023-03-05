mod algo;
mod graph;
mod util;

use std::collections::HashMap;

use algo::maze::MazeGenerate;
use image::GenericImageView;
use util::build_offset_getter;

use crate::{algo::maze::Direction, graph::builder::GraphBuilder};

fn is_corridor(floors: [&bool; 4]) -> bool {
    let left_right = [false, true, false, true];
    let top_bottom = [true, false, true, false];
    let floors = floors.map(|b| *b);

    floors == left_right || floors == top_bottom
}

fn main() {
    let mut maze_algo = algo::RandomisedDFS::from_grid_size(10, 10);
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

    let mut pixel_map = HashMap::<(u32, u32), bool>::new();
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if pixel.0 == [255u8] || pixel.0 == [123u8] {
            pixel_map.insert((x, y), true);
            pixel.0 = [255u8];
        }
    }

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if pixel.0 == [255u8] {
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
        if pixel.0 == [255u8] {
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

            if x == 7 && y == 5 {
                println!("beans");
            }

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
    let path = &graph.find_path((1, 1), (19, 19));

    println!("Path possibly found");

    if let Some(path) = path {
        path.iter().for_each(|(x, y)| {
            let pixel = image.get_pixel_mut(*x, *y);
            pixel.0 = [80u8];
        });
    }

    image.save("maze.png").unwrap();

    println!("Maze nodes saved");
}
