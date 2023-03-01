use crate::algo::maze::Direction;
use std::ops::{Add, Sub};

pub fn build_offset_getter<T: PartialEq + Sub<u32, Output = T> + Add<u32, Output = T> + Copy>(
    min: (T, T),
    max: (T, T),
) -> impl Fn(T, T, Direction) -> Option<(T, T)> {
    move |x: T, y: T, dir: Direction| {
        // (row, col)
        let index_tuple: (T, T) = match dir {
            Direction::Top => {
                if y == min.1 {
                    return None;
                }

                (y - 1, x)
            }
            Direction::Right => {
                if x == max.0 - 1 {
                    return None;
                }

                (y, x + 1)
            }
            Direction::Bottom => {
                if y == max.1 - 1 {
                    return None;
                }

                (y + 1, x)
            }
            Direction::Left => {
                if x == min.0 {
                    return None;
                }

                (y, x - 1)
            }
        };

        Some(index_tuple)
    }
}
