mod algo;
mod graph;

use algo::maze::MazeGenerate;

fn main() {
    let mut maze_algo = algo::RandomisedDFS::from_grid_size(200, 200);
    maze_algo.generate();

    let image = maze_algo.grid.generate_as_image();

    image.save("maze.png").unwrap();
}
