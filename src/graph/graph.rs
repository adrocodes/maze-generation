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
        let mut nodes_checked = 0;
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
            nodes_checked += 1;
            if let Some(v) = stack.pop_front() {
                if v == end {
                    println!("Nodes Checked: {}", nodes_checked);
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
    pub fn astar(
        &self,
        start: T,
        end: T,
        heuristic_fn: &dyn Fn(&T, &T) -> i32,
        distance_fn: &dyn Fn(&T, &T) -> i32,
    ) {
        let mut nodes_checked = 0;
        let mut queue = BinaryHeap::<QueueItem<T>>::new();

        let mut g_score = HashMap::<T, i32>::new();
        g_score.insert(start.clone(), 0);

        queue.push(QueueItem {
            cost: *g_score.get(&start).unwrap_or(&0),
            position: start.clone(),
        });

        while let Some(QueueItem { cost: _, position }) = queue.pop() {
            nodes_checked += 1;
            if position == end {
                break;
            }

            if let Some(edges) = self.vertices.get(&position) {
                for neighbor in edges.iter() {
                    let current_g_score = *g_score.get(&position).unwrap();
                    let tentative_g_score = current_g_score + distance_fn(&position, &neighbor);

                    if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&i32::MAX) {
                        let mut path = self.path.borrow_mut();
                        path.insert(
                            neighbor.clone(),
                            Node {
                                value: neighbor.clone(),
                                parent: Some(position.clone()),
                                children: None,
                            },
                        );

                        g_score.insert(neighbor.clone(), tentative_g_score);

                        queue.push(QueueItem {
                            cost: tentative_g_score + heuristic_fn(&neighbor, &end),
                            position: neighbor.clone(),
                        })
                    }
                }
            }
        }

        println!("Nodes Checked: {}", nodes_checked);
    }
}
