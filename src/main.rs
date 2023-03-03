mod algo;
mod graph;
mod util;

use std::collections::HashMap;

use algo::maze::MazeGenerate;
use image::GenericImageView;
use util::build_offset_getter;

use crate::{algo::maze::Direction, graph::builder::GraphBuilder};

fn is_corridor(walls: [&bool; 4]) -> bool {
    let left_right = [false, true, false, true];
    let top_bottom = [true, false, true, false];
    let walls = walls.map(|b| *b);

    walls == left_right || walls == top_bottom
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
    for (x, y, pixel) in image.enumerate_pixels() {
        if pixel.0 == [255u8] {
            pixel_map.insert((x, y), true);
        }
    }

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if pixel.0 == [255u8] {
            let walls = [
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

            if !is_corridor(walls) {
                pixel.0 = [123u8];
                builder.add_node((x, y));
            }
        }
    }

    let graph = builder.build();

    dbg!(&graph.vertices);

    println!("Nodes generated");

    image.save("maze.png").unwrap();

    println!("Maze nodes saved");
}
