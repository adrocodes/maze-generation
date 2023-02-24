mod algo;

use algo::maze::MazeGenerate;

fn main() {
    let maze_algo = algo::RandomisedDFS;
    maze_algo.generate();
}
