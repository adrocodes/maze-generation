use std::{
    cell::RefCell,
    cmp::{Eq, PartialEq},
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

use super::graph::{Graph, Node};

#[derive(Debug)]
pub struct GraphBuilder<T: PartialEq + Eq + Hash + Clone + Debug> {
    pub vertices: HashMap<T, HashSet<T>>,
}

impl<T: PartialEq + Eq + Hash + Clone + Debug> Default for GraphBuilder<T> {
    fn default() -> Self {
        GraphBuilder {
            vertices: HashMap::new(),
        }
    }
}

impl<T: PartialEq + Eq + Hash + Clone + Debug> GraphBuilder<T> {
    pub fn new() -> Self {
        GraphBuilder::default()
    }

    fn _insert_node(builder: &mut GraphBuilder<T>, node: &T) {
        if !builder.vertices.contains_key(node) {
            builder.vertices.insert(node.clone(), HashSet::new());
        }
    }

    pub fn insert_node(mut self, node: T) -> GraphBuilder<T> {
        GraphBuilder::<T>::_insert_node(&mut self, &node);
        self
    }

    pub fn add_node(&mut self, node: T) {
        GraphBuilder::<T>::_insert_node(self, &node);
    }

    fn _insert_edge(builder: &mut GraphBuilder<T>, from: &T, to: &T) {
        let has_from = builder.vertices.contains_key(&from);
        let has_to = builder.vertices.contains_key(&to);

        if !has_from || !has_to {
            return;
        }

        if let Some(from_vert) = builder.vertices.get_mut(&from) {
            from_vert.insert(to.clone());
        }

        if let Some(to_vert) = builder.vertices.get_mut(&to) {
            to_vert.insert(from.clone());
        }
    }

    pub fn insert_edge(mut self, from: T, to: T) -> GraphBuilder<T> {
        GraphBuilder::<T>::_insert_edge(&mut self, &from, &to);

        self
    }

    pub fn add_edge(&mut self, from: T, to: T) {
        GraphBuilder::<T>::_insert_edge(self, &from, &to);
    }

    pub fn from_nodes(nodes: Vec<T>) -> GraphBuilder<T> {
        let mut graph = GraphBuilder::<T>::new();

        for node in nodes {
            GraphBuilder::<T>::_insert_node(&mut graph, &node);
        }

        graph
    }

    pub fn from_edges(edges: Vec<(T, T)>) -> GraphBuilder<T> {
        let mut graph = GraphBuilder::<T>::new();

        for (from, to) in edges {
            GraphBuilder::<T>::_insert_node(&mut graph, &from);
            GraphBuilder::<T>::_insert_node(&mut graph, &to);
            GraphBuilder::<T>::_insert_edge(&mut graph, &from, &to);
        }

        graph
    }

    pub fn build(self) -> Graph<T> {
        Graph {
            vertices: self.vertices,
            path: RefCell::new(HashMap::<T, Node<T>>::new()),
        }
    }
}
