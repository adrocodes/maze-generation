use std::{
    cell::{Ref, RefCell},
    cmp::{Eq, PartialEq},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
};

type Path<T> = HashMap<T, Node<T>>;

#[derive(Debug)]
pub struct Graph<T: PartialEq + Eq + Hash + Clone + Debug + Ord> {
    pub vertices: HashMap<T, HashSet<T>>,
    pub path: RefCell<Path<T>>,
}

#[derive(Debug)]
pub struct Node<T: PartialEq + Eq + Hash + Clone + Debug + Ord> {
    pub value: T,
    pub parent: Option<T>,
    pub children: Option<Vec<T>>,
}

#[derive(Clone, Eq, PartialEq)]
struct QueueItem<T: PartialEq + Eq + Hash + Clone + Debug + Ord> {
    cost: i32,
    position: T,
}

impl<T> Ord for QueueItem<T>
where
    T: PartialEq + Eq + Hash + Clone + Debug + Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl<T> PartialOrd for QueueItem<T>
where
    T: PartialEq + Eq + Hash + Clone + Debug + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Graph<T>
where
    T: PartialEq + Eq + Hash + Clone + Debug + Ord,
{
    pub fn get_path(&self) -> Ref<Path<T>> {
        self.path.borrow()
    }

    pub fn bfs(&self, start: T, end: T) -> Option<Path<T>> {
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

    /// Based on https://en.wikipedia.org/wiki/A*_search_algorithm#Pseudocode
    pub fn astar(&self, start: T, end: T, heuristic_fn: &dyn Fn(T, T) -> i32) {
        // create a priority queue with only start in it
        let mut queue = BinaryHeap::<QueueItem<T>>::new();

        // create a mut hashmap for gScore
        // set start value 0

        // create a mut hashmap for fScore
        // set start to h(start, end)

        // while priority queue isn't empty
    }
}
