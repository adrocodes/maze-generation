use super::maze::MazeGenerate;

/// Based off this description and implementation description:
/// [Wikipedia](https://en.wikipedia.org/wiki/Maze_generation_algorithm#Randomized_depth-first_search)
pub struct RandomisedDFS;

impl MazeGenerate for RandomisedDFS {
    fn generate(self) -> Self {
        self
    }
}
