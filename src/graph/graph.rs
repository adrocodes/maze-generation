use std::{
    cmp::{Eq, PartialEq},
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
};

#[derive(Debug)]
pub struct Graph<T: PartialEq + Eq + Hash + Clone + Debug> {
    pub vertices: HashMap<T, HashSet<T>>,
}

#[derive(Debug)]
pub struct Node<T: PartialEq + Eq + Hash + Clone + Debug> {
    pub value: T,
    pub parent: Option<T>,
    pub children: Option<Vec<T>>,
}

impl<T: PartialEq + Eq + Hash + Clone + Debug> Graph<T> {
    pub fn bfs(&self, start: T, end: T) -> Option<(T, Vec<Node<T>>)> {
        let mut stack = VecDeque::<T>::new();
        let mut visited = HashSet::<T>::new();
        let mut path = Vec::<Node<T>>::new();

        visited.insert(start.clone());
        stack.push_front(start.clone());
        path.push(Node {
            value: start,
            parent: None,
            children: None,
        });

        while !stack.is_empty() {
            if let Some(v) = stack.pop_front() {
                if v == end {
                    return Some((v, path));
                } else {
                    if let Some(edges) = self.vertices.get(&v) {
                        for w in edges.iter() {
                            if !visited.contains(&w) {
                                visited.insert(w.clone());
                                stack.push_back(w.clone());
                                path.push(Node {
                                    value: w.clone(),
                                    parent: Some(v.clone()),
                                    children: None,
                                });
                            }
                        }
                    }
                }
            }
        }

        None
    }
}
