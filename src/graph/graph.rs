use std::{
    cmp::{Eq, PartialEq},
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
};

#[derive(Debug)]
pub struct Graph<T: PartialEq + Eq + Hash + Clone + Debug> {
    pub vertices: HashMap<T, HashSet<T>>,
    pub path: Option<HashMap<T, Node<T>>>,
}

#[derive(Debug)]
pub struct Node<T: PartialEq + Eq + Hash + Clone + Debug> {
    pub value: T,
    pub parent: Option<T>,
    pub children: Option<Vec<T>>,
}

impl<T: PartialEq + Eq + Hash + Clone + Debug> Graph<T> {
    pub fn bfs(&self, start: T, end: T) -> Option<HashMap<T, Node<T>>> {
        let mut stack = VecDeque::<T>::new();
        let mut visited = HashSet::<T>::new();
        let mut map_path = HashMap::<T, Node<T>>::new();

        visited.insert(start.clone());
        stack.push_front(start.clone());
        map_path.insert(
            start.clone(),
            Node {
                value: start.clone(),
                parent: None,
                children: None,
            },
        );

        while !stack.is_empty() {
            if let Some(v) = stack.pop_front() {
                if v == end {
                    return Some(map_path);
                } else {
                    if let Some(edges) = self.vertices.get(&v) {
                        for w in edges.iter() {
                            if !visited.contains(&w) {
                                visited.insert(w.clone());
                                stack.push_back(w.clone());
                                map_path.insert(
                                    w.clone(),
                                    Node {
                                        value: w.clone(),
                                        parent: Some(v.clone()),
                                        children: None,
                                    },
                                );
                            }
                        }
                    }
                }
            }
        }

        None
    }

    pub fn astar(
        &mut self,
        start: T,
        end: T,
        distance_fn: &dyn Fn(T, T) -> i32,
    ) -> Option<HashMap<T, Node<T>>> {
        None
    }
}
