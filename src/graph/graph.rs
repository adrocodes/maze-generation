use std::{
    cmp::{Eq, PartialEq},
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

#[derive(Debug)]
pub struct Graph<T: PartialEq + Eq + Hash + Clone + Debug> {
    pub vertices: HashMap<T, HashSet<T>>,
}

impl<T: PartialEq + Eq + Hash + Clone + Debug> Graph<T> {
    fn _traverse(
        graph: &Graph<T>,
        path: &mut Vec<T>,
        node: &T,
        current: &HashSet<T>,
        goal: &T,
        visited: &mut HashSet<T>,
        found: &mut bool,
    ) -> Vec<T> {
        visited.insert(node.clone());

        if *found {
            return path.to_vec();
        }

        if *node == *goal {
            path.push(node.clone());
            *found = true;
        } else if current.contains(&goal) {
            path.push(goal.clone());
            *found = true;
        } else {
            for n in current.iter() {
                if visited.contains(&n) || *found {
                    continue;
                }

                if *n == *goal {
                    path.push(n.clone());
                } else {
                    let current = graph.vertices.get(&n).unwrap();
                    let mut new_path =
                        Graph::<T>::_traverse(&graph, path, &n, &current, goal, visited, found);

                    let last = new_path.last();

                    if let Some(last) = last {
                        if *last == *goal && *last != *n {
                            let mut last_vec = new_path.split_off(1);
                            new_path.push(n.clone());
                            new_path.append(&mut last_vec);
                            *path = new_path;
                            *found = true;
                            break;
                        }
                    }
                }
            }
        }

        path.to_vec()
    }

    pub fn find_path(&self, start: T, end: T) -> Option<Vec<T>> {
        let mut path: Vec<T> = vec![];
        let start_node = self.vertices.get(&start);

        if start_node.is_none() {
            return None;
        }

        let start_node = start_node.unwrap();

        if start == end {
            return Some(vec![start]);
        }

        path.push(start.clone());
        let mut visited = HashSet::<T>::new();
        visited.insert(start.clone());
        let mut found = false;

        path = Graph::<T>::_traverse(
            &self,
            &mut path,
            &start,
            start_node,
            &end,
            &mut visited,
            &mut found,
        );

        if !found {
            return None;
        }

        Some(path)
    }
}
