use std::fmt::{Display, Formatter, Result};
use std::hash::Hash;
use std::hash::Hasher;

use crate::ravioli::O;

pub mod astar;
// pub mod astar_vec;

fn simple_hash<T: Hash>(object: &T) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    object.hash(&mut hasher);
    hasher.finish()
}

pub trait SearchInfo<T: State>: std::fmt::Debug {
    fn heuristic(&self, _state: &T) -> u64 {
        0
    }
    fn max_depth(&self) -> Option<u64> {
        None
    }
    fn expand_state(&self, state: &T) -> Vec<T>;
    fn is_goal(&self, state: &T) -> bool;
}

#[derive(Debug)]
pub struct SearchNode<'a, T: State> {
    to_root: Option<O<SearchNode<'a, T>>>,
    level: u64,
    pub state: T,
    pub cached_state_hash: u64,

    // https://stackoverflow.com/questions/34028324/how-do-i-use-a-custom-comparator-function-with-btreeset
    // https://stackoverflow.com/questions/35786878/how-can-i-implement-ord-when-the-comparison-depends-on-data-not-part-of-the-comp/35788530#35788530
    search: &'a dyn SearchInfo<T>,
}

impl<'a, T: State + Display> Display for SearchNode<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} - {}", self.level, self.state)
    }
}

impl<'a, T: State> SearchNode<'a, T> {
    fn new_root(state: T, search: &'a dyn SearchInfo<T>) -> Self {
        SearchNode {
            to_root: None,
            level: 0,
            cached_state_hash: simple_hash(&state),
            state: state,
            search: search,
        }
    }
}

pub trait State: Eq + Hash + Clone {}

pub fn new_child<'a, T: State>(node: &O<SearchNode<'a, T>>, new_state: T) -> SearchNode<'a, T> {
    SearchNode {
        to_root: Some(node.clone()),
        level: node.borrow().level + 1,
        cached_state_hash: simple_hash(&new_state),
        state: new_state,
        search: node.borrow().search,
    }
}

pub fn root_path<'a, T: State>(node: &'a O<SearchNode<'a, T>>) -> Vec<O<SearchNode<'a, T>>> {
    let mut ret: Vec<O<SearchNode<'a, T>>> = Vec::new();
    let cloned = node.clone();
    let mut option: Option<O<SearchNode<'a, T>>> = Some(cloned);

    while option.is_some() {
        let o = option.unwrap();
        option = o.borrow().to_root.clone();
        ret.push(o.clone());
    }

    ret
}

pub fn root_path_state<'a, T: State>(node: &'a O<SearchNode<'a, T>>) -> Vec<T> {
    let path = root_path(node);
    let mut ret: Vec<T> = path.iter().map(|o| o.borrow().state.clone()).collect();
    ret.reverse();
    ret
}

fn expand_node<'a, T: State>(node: &O<SearchNode<'a, T>>) -> Vec<O<SearchNode<'a, T>>> {
    let search_data = node.borrow().search;
    let childs = search_data.expand_state(&node.borrow().state);
    childs
        .iter()
        .map(|c| new_child(&node, c.clone()))
        .map(|c| O::new(c))
        .collect()
}

pub fn deep_first_search<'a, T: State + std::fmt::Debug>(
    root: T,
    search_data: &'a dyn SearchInfo<T>,
) -> Option<O<SearchNode<'a, T>>> {
    fn search<'a, T: State + std::fmt::Debug>(
        current: &O<SearchNode<'a, T>>,
    ) -> Option<O<SearchNode<'a, T>>> {
        let state: &T = &current.borrow().state;

        println!("level: {} state: {:?}", current.borrow().level, state);

        let search_data = current.borrow().search;
        if search_data.is_goal(state) {
            return Some(current.clone());
        }

        match search_data.max_depth() {
            Some(max) => {
                if current.borrow().level >= max {
                    return None;
                }
            }
            None => {}
        }

        let children = expand_node(current);
        for child in children {
            let ret = search(&child);
            if ret.is_some() {
                return ret;
            }
        }
        None
    }

    let root = SearchNode::new_root(root, search_data);
    search(&O::new(root))
}

pub fn breadth_first_search<'a, T: State + std::fmt::Debug>(
    root: T,
    search_data: &'a dyn SearchInfo<T>,
) -> Option<O<SearchNode<'a, T>>> {
    use std::collections::VecDeque;

    let mut queue: VecDeque<O<SearchNode<T>>> = VecDeque::new();

    fn search<'a, T: State + std::fmt::Debug>(
        queue: &mut VecDeque<O<SearchNode<'a, T>>>,
    ) -> Option<O<SearchNode<'a, T>>> {
        while let Some(current_node) = queue.pop_back() {
            let state = &current_node.borrow().state;
            println!("level: {} state: {:?}", current_node.borrow().level, state);

            let search_data = current_node.borrow().search;
            if search_data.is_goal(state) {
                println!("found: state: {:?}", state);
                return Some(current_node.clone());
            }

            if let Some(max) = search_data.max_depth() {
                if current_node.borrow().level >= max {
                    println!("  max level reached:{}", max);
                    return None;
                }
            }
            let children = expand_node(&current_node);
            for child in children {
                println!(" child: {:?}", child);
                queue.push_front(child)
            }
        }
        None
    }

    let root = SearchNode::new_root(root, search_data);
    queue.push_back(O::new(root));
    search(&mut queue)
}

#[cfg(test)]
mod tests {

    use crate::search::*;
    use std::ops::Deref;

    impl State for Vec<i32> {}

    impl Display for O<SearchNode<'_, Vec<i32>>> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let v: &Vec<i32> = &self.borrow().state;
            let v: Vec<String> = v.iter().map(|i| i.to_string()).collect();
            write!(f, "O({})", v.join("-"))
        }
    }

    #[derive(Debug)]
    struct DummySearch {}

    impl SearchInfo<Vec<i32>> for DummySearch {
        fn expand_state(&self, state: &Vec<i32>) -> Vec<Vec<i32>> {
            if state.len() < 4 {
                [0, 1, 2, 3]
                    .iter()
                    .map(|i| {
                        let mut child = state.clone();
                        child.push(*i);
                        child
                    })
                    .collect()
            } else {
                Vec::new()
            }
        }

        fn is_goal(&self, state: &Vec<i32>) -> bool {
            *state == vec![0 as i32, 1 as i32, 2 as i32, 3 as i32]
        }
    }

    #[test]
    fn deep_first_search_test() {
        let root = vec![];
        let goal = deep_first_search(root.clone(), &DummySearch {});

        assert!(goal.is_some());
        let goal = goal.unwrap();
        let path = root_path_state(&goal);
        println!("{:?}", path);
        assert!(path[0] == root)
    }

    #[test]
    fn breadth_first_search_test() {
        let root = vec![];
        let goal = breadth_first_search(root.clone(), &DummySearch {});

        assert!(goal.is_some());

        let goal = goal.unwrap();
        let path = root_path_state(&goal);
        println!("{:?}", path);
        assert!(path[0] == root);
        assert!(DummySearch {}.is_goal(&path[path.len() - 1]));
    }

    #[test]
    fn expand_test() {
        let vec = vec![0];
        let children = DummySearch {}.expand_state(&vec);
        println!("{:?}", children);
        assert!(children == vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3]]);
    }

    #[test]
    fn expand_node_test() {
        let vec = vec![0];
        let node = O::new(SearchNode::new_root(vec.clone(), &DummySearch {}));
        let children = expand_node(&node);

        println!(
            "{}",
            children
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
        assert!(children.len() == 4);

        let children = expand_node(&children[0]);
        println!(
            "{}",
            children
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
        assert!(children.len() == 4);
    }

    #[test]
    fn root_path_test() {
        let vec = vec![0];
        let node = O::new(SearchNode::new_root(vec, &DummySearch {}));
        let children = expand_node(&node);
        let children = expand_node(&children[0]);
        let children = expand_node(&children[0]);

        let root_path = root_path(&children[0]);

        println!(
            "{}",
            root_path
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
        assert!(root_path.len() == 4);

        let root: std::cell::Ref<SearchNode<Vec<i32>>> = root_path[root_path.len() - 1].borrow();

        assert!(std::ptr::eq(root.deref(), node.borrow().deref()));
    }

    #[test]
    fn is_goal_test() {
        let search = DummySearch {};
        assert!(!search.is_goal(&vec![0]));
        assert!(search.is_goal(&vec![0, 1, 2, 3]));
    }
}
