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
}

impl<T: PartialEq + Eq + Hash + Clone + Debug> Graph<T> {
    pub fn bfs(&self, start: T, end: T) -> Option<Node<T>> {
        let mut stack = VecDeque::<Node<T>>::new();
        let mut visited = HashSet::<T>::new();

        visited.insert(start.clone());
        stack.push_front(Node {
            value: start,
            parent: None,
        });

        while !stack.is_empty() {
            if let Some(v) = stack.pop_front() {
                if v.value == end {
                    return Some(v);
                } else {
                    if let Some(edges) = self.vertices.get(&v.value) {
                        for w in edges.iter() {
                            if !visited.contains(&w) {
                                visited.insert(w.clone());
                                stack.push_back(Node {
                                    value: w.clone(),
                                    parent: Some(v.value.clone()),
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
