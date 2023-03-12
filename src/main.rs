mod algo;
mod graph;
mod util;

use std::collections::HashMap;

use algo::maze::MazeGenerate;
use graph::graph::Node;
use image::{GenericImageView, ImageBuffer, Luma};
use util::build_offset_getter;

type Point = (u32, u32);
type Path = HashMap<Point, Node<Point>>;
type MazeImage = ImageBuffer<Luma<u8>, Vec<u8>>;
type Floors = [bool; 4];

use crate::{algo::maze::Direction, graph::builder::GraphBuilder};

const SOLUTION_PATH_COLOUR: [u8; 1] = [100u8];
const PATH_COLOUR: [u8; 1] = [255u8];
const MAZE_SIZE: (usize, usize) = (500, 500);
const STARTING_SPOT: Point = (1, 0);
const ENDING_SPOT: Point = ((MAZE_SIZE.0 as u32 * 2) - 1, MAZE_SIZE.1 as u32 * 2);

fn is_corridor(floors: Floors) -> bool {
    let left_right = [false, true, false, true];
    let top_bottom = [true, false, true, false];

    floors == left_right || floors == top_bottom
}

fn find_in_path(path: &Path, value: Point) -> Option<&Node<Point>> {
    path.get(&value)
}

fn draw_solution(image: &mut MazeImage, path: &Path, start: Point) {
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

fn add_maze_start(image: &mut MazeImage) {
    let mut pixel = image.get_pixel_mut(STARTING_SPOT.0, STARTING_SPOT.1);
    pixel.0 = PATH_COLOUR;
}

fn add_maze_end(image: &mut MazeImage) {
    let mut pixel = image.get_pixel_mut(ENDING_SPOT.0, ENDING_SPOT.1);
    pixel.0 = PATH_COLOUR;
}

fn find_neighboring_nodes(
    builder: &mut GraphBuilder<Point>,
    pixel_map: &HashMap<Point, bool>,
    offset_getter: &dyn Fn(u32, u32, Direction) -> Option<Point>,
    x: u32,
    y: u32,
    direction: Direction,
) {
    let mut offset_cell = offset_getter(x, y, direction);

    while let Some(cell) = offset_cell {
        offset_cell = offset_getter(cell.0, cell.1, direction);

        if !pixel_map.get(&(cell.0, cell.1)).unwrap_or(&false) {
            offset_cell = None;
        } else if builder.vertices.contains_key(&(cell.0, cell.1)) {
            builder.add_edge((cell.0, cell.1), (x, y));
            offset_cell = None;
        }
    }
}

fn get_surrounding_floors(
    pixel_map: &HashMap<Point, bool>,
    offset_getter: &dyn Fn(u32, u32, Direction) -> Option<Point>,
    x: u32,
    y: u32,
) -> Floors {
    let floors = [
        pixel_map
            .get(&offset_getter(x, y, Direction::Top).unwrap_or((0, 0)))
            .unwrap_or(&false)
            .clone(),
        pixel_map
            .get(&offset_getter(x, y, Direction::Right).unwrap_or((0, 0)))
            .unwrap_or(&false)
            .clone(),
        pixel_map
            .get(&offset_getter(x, y, Direction::Bottom).unwrap_or((0, 0)))
            .unwrap_or(&false)
            .clone(),
        pixel_map
            .get(&offset_getter(x, y, Direction::Left).unwrap_or((0, 0)))
            .unwrap_or(&false)
            .clone(),
    ];

    floors
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
    let mut builder = GraphBuilder::<Point>::new();

    add_maze_start(image);
    add_maze_end(image);

    let mut pixel_map = HashMap::<Point, bool>::new();

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if pixel.0 == PATH_COLOUR || pixel.0 == SOLUTION_PATH_COLOUR {
            pixel_map.insert((x, y), true);
            pixel.0 = PATH_COLOUR;
        }
    }

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if pixel.0 == PATH_COLOUR {
            let floors = get_surrounding_floors(&pixel_map, &offset_getter, x, y);

            if !is_corridor(floors) {
                builder.add_node((x, y));

                if floors[0] {
                    find_neighboring_nodes(
                        &mut builder,
                        &pixel_map,
                        &offset_getter,
                        x,
                        y,
                        Direction::Top,
                    );
                }

                if floors[3] {
                    find_neighboring_nodes(
                        &mut builder,
                        &pixel_map,
                        &offset_getter,
                        x,
                        y,
                        Direction::Left,
                    );
                }
            }
        }
    }

    println!("Nodes generated");

    let graph = builder.build();

    println!("Number of ndoes: {}", graph.vertices.len());
    println!("Has end: {:?}", graph.vertices.contains_key(&ENDING_SPOT));

    let path = &graph.bfs(STARTING_SPOT, ENDING_SPOT);

    if let Some(path) = path {
        println!("Path found - drawing solution");
        println!("Path length: {}", path.len());

        draw_solution(image, path, ENDING_SPOT);

        println!("Path completed");
    } else {
        println!("Path not found");
    }

    image.save("maze.png").unwrap();

    println!("Maze nodes saved");
}
