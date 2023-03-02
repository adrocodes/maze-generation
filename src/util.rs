use crate::algo::maze::Direction;
use std::ops::{Add, Sub};

pub fn build_offset_getter<T: PartialEq + Sub<u32, Output = T> + Add<u32, Output = T> + Copy>(
    min: (T, T),
    max: (T, T),
) -> impl Fn(T, T, Direction) -> Option<(T, T)> {
    move |x: T, y: T, dir: Direction| {
        // (x, y)
        let index_tuple: (T, T) = match dir {
            Direction::Top => {
                if y == min.1 {
                    return None;
                }

                (x, y - 1)
            }
            Direction::Right => {
                if x == max.0 - 1 {
                    return None;
                }

                (x + 1, y)
            }
            Direction::Bottom => {
                if y == max.1 - 1 {
                    return None;
                }

                (x, y + 1)
            }
            Direction::Left => {
                if x == min.0 {
                    return None;
                }

                (x - 1, y)
            }
        };

        Some(index_tuple)
    }
}

mod test {
    use super::build_offset_getter;
    use crate::algo::maze::Direction;

    #[test]
    fn test_top_corner() {
        let get = build_offset_getter((0, 0), (10, 10));

        let top = get(0, 0, Direction::Top);
        assert!(top.is_none());

        let right = get(0, 0, Direction::Right);
        assert!(right.is_some());
        assert_eq!((0, 1), right.unwrap());

        let bottom = get(0, 0, Direction::Bottom);
        assert!(bottom.is_some());
        assert_eq!((1, 0), bottom.unwrap());

        let left = get(0, 0, Direction::Left);
        assert!(left.is_none());
    }

    #[test]
    fn test_all_dirs_corner() {
        let get = build_offset_getter((0, 0), (10, 10));

        let top = get(4, 4, Direction::Top);
        assert_eq!((3, 4), top.unwrap());

        let right = get(4, 4, Direction::Right);
        assert_eq!((4, 5), right.unwrap());

        let bottom = get(4, 4, Direction::Bottom);
        assert_eq!((5, 4), bottom.unwrap());

        let left = get(4, 4, Direction::Left);
        assert_eq!((4, 3), left.unwrap());
    }
}
