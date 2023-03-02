mod algo;
mod graph;
mod util;

use std::collections::HashMap;

use algo::maze::MazeGenerate;
use image::GenericImageView;
use util::build_offset_getter;

use crate::algo::maze::Direction;

fn is_corridor(walls: [(u32, u32); 4], pixels: &HashMap<(u32, u32), bool>) -> bool {
    let top = pixels.get(&walls[0]).unwrap_or(&false);
    let right = pixels.get(&walls[1]).unwrap_or(&false);
    let bottom = pixels.get(&walls[2]).unwrap_or(&false);
    let left = pixels.get(&walls[3]).unwrap_or(&false);

    (*top == false && *bottom == false && *right == true && *left == true)
        || (*top == true && *bottom == true && *right == false && *left == false)
}

fn main() {
    let mut maze_algo = algo::RandomisedDFS::from_grid_size(5000, 5000);
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

    let mut pixel_map = HashMap::<(u32, u32), bool>::new();
    for (x, y, pixel) in image.enumerate_pixels() {
        if pixel.0 == [255u8] {
            pixel_map.insert((x, y), true);
        }
    }

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if pixel.0 == [255u8] {
            let walls = [
                offset_getter(x, y, Direction::Top).unwrap_or((0, 0)),
                offset_getter(x, y, Direction::Right).unwrap_or((0, 0)),
                offset_getter(x, y, Direction::Bottom).unwrap_or((0, 0)),
                offset_getter(x, y, Direction::Left).unwrap_or((0, 0)),
            ];

            if !is_corridor(walls, &pixel_map) {
                pixel.0 = [123u8];
            }
        }
    }

    println!("Nodes generated");

    image.save("maze.png").unwrap();

    println!("Maze nodes saved");
}
