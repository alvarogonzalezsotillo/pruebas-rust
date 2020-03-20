use std::cmp::Ordering;
use std::cell::Ref;
use std::fmt::{Display, Formatter, Result};
use std::ops::Deref;
use std::ops::DerefMut;

use crate::ravioli::O;

#[derive(Debug)]
struct SearchNode<T:State>{
    to_root : Option<O<SearchNode<T>>>,
    level : u16,
    state : T
}


impl <T:State + Display> Display for SearchNode<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} - {}", self.level, self.state )
    }
}


impl <T:State> SearchNode<T>{
    fn new_root(state: T) -> Self{
        SearchNode{
            to_root : None,
            level : 0,
            state : state
        }
    }
}

trait State : Clone{
    fn expand_state(&self) -> Vec<Self>;
    fn is_goal(&self) -> bool;
}

fn new_child<T:State>(node : &O<SearchNode<T>>, new_state: T) -> SearchNode<T>{
    SearchNode{
        to_root: Some(node.clone()),
        level : node.borrow().level+1,
        state: new_state
    }
}

fn root_path<T:State>(node: &O<SearchNode<T>> ) -> Vec<O<SearchNode<T>>>{
    let mut ret : Vec<O<SearchNode<T>>> = Vec::new();
    let mut option : Option<O<SearchNode<T>>> = Some(node.clone());
        
    while option.is_some() {
        let o = option.unwrap();
        option = o.borrow().to_root.clone();
        ret.push( o.clone() );
    }

    ret
    
}

fn root_path_state<T:State>(node: &O<SearchNode<T>>) -> Vec<T> {
    let path = root_path(node);
    path.iter().map( |o| o.borrow().state.clone() ).collect()
}


fn expand_node<T:State>(node: &O<SearchNode<T>>) -> Vec<O<SearchNode<T>>>{
    let childs = node.borrow().state.expand_state();
    childs.
        iter().
        map( |c| new_child(&node, c.clone() ) ).
        map( |c| O::new(c) ).
        collect()
}


struct Search<T:State>{
    root : T,
    all_nodes : std::collections::HashMap<T,O<SearchNode<T>>>,
    not_expanded_nodes : std::collections::BTreeSet<O<SearchNode<T>>>,
}

fn deep_first_search<T:State + std::fmt::Debug>(root:T) -> Option<O<SearchNode<T>>>{

    fn search<T:State + std::fmt::Debug>( current: &O<SearchNode<T>> ) -> Option<O<SearchNode<T>>> {

        
        let state : &T = &current.borrow().state;

        println!("level: {} state: {:?}", current.borrow().level, state );

        
        if state.is_goal() {
            return Some(current.clone());
        }

        let children = expand_node(current);
        for child in children{
            let ret = search(&child);
            if ret.is_some(){
                return ret;
            }
        };
        None
    }
    
    let root = SearchNode::new_root(root);
    search(&O::new(root))
}



fn breadth_first_search<T:State + std::fmt::Debug>(root:T) -> Option<O<SearchNode<T>>>{

    use std::collections::VecDeque;
    
    let mut queue : VecDeque<O<SearchNode<T>>> = VecDeque::new();
    
    fn search<T:State + std::fmt::Debug>( queue: &mut VecDeque<O<SearchNode<T>>> ) -> Option<O<SearchNode<T>>> {

        
        while let Some(current_node) = queue.pop_back() {
            let state = &current_node.borrow().state;
            println!("level: {} state: {:?}", current_node.borrow().level, state );

            if state.is_goal() {
                return Some(current_node.clone());
            }

            let children = expand_node(&current_node);
            for child in children {
                queue.push_front(child)
            }
        }
        None
    }
    
    let root = SearchNode::new_root(root);
    queue.push_back(O::new(root));
    search(&mut queue)
}



#[cfg(test)]
mod tests{

    use crate::search::*;

    impl State for Vec<i32>{
        fn expand_state(&self) -> Vec<Vec<i32>> {

            if self.len() < 4 {
                
                [0,1,2,3].
                    iter().
                    map( |i| {
                        let mut child = self.clone();
                        child.push(*i);
                        child
                    }).
                    collect()
            }
            else{
                Vec::new()
            }
        }

        fn is_goal(&self) -> bool {
            *self == vec![0 as i32,1 as i32,2 as i32,3 as i32]
        }
    }


    impl Display for O<SearchNode<Vec<i32>>>{
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let v : &Vec<i32> = &self.borrow().state;
            let v : Vec<String> = v.iter().map(|i| i.to_string() ).collect();
            write!(f, "O({})", v.join("-") )
        }
    }

    #[test]
    fn deep_first_search_test(){
        let root = vec![];
        let goal = deep_first_search(root.clone());

        assert!( goal.is_some() );
        let goal = goal.unwrap();
        let path = root_path_state(&goal);
        println!( "{:?}", path );
        assert!( path[path.len()-1] == root )
        
    }


    #[test]
    fn breadth_first_search_test(){
        let root = vec![];
        let goal = breadth_first_search(root.clone());

        assert!( goal.is_some() );
        
        let goal = goal.unwrap();
        let path = root_path_state(&goal);
        println!( "{:?}", path );
        assert!( path[path.len()-1] == root )
    }

    
    #[test]
    fn expand_test(){
        let vec = vec![0];
        let children = vec.expand_state();
        println!("{:?}", children );
        assert!(children == vec![ vec![0,0], vec![0,1], vec![0,2], vec![0,3] ]);
    }

    #[test]
    fn expand_node_test(){
        let vec = vec![0];
        let node = O::new(SearchNode::new_root(vec.clone()));
        let children = expand_node(&node);

        println!("{}", children.iter().map(|c| c.to_string() ).collect::<Vec<String>>().join(" ") );
        assert!( children.len() == 4 );

        let children = expand_node(&children[0]);
        println!("{}", children.iter().map(|c| c.to_string() ).collect::<Vec<String>>().join(" ") );
        assert!( children.len() == 4 );
    }
    

    #[test]
    fn root_path_test(){
        let vec = vec![0];
        let node = O::new(SearchNode::new_root(vec));
        let children = expand_node(&node);
        let children = expand_node(&children[0]);
        let children = expand_node(&children[0]);

        let root_path = root_path(&children[0]);
        
        println!("{}", root_path.iter().map(|c| c.to_string() ).collect::<Vec<String>>().join(" ") );
        assert!( root_path.len() == 4 );

        let root : Ref<SearchNode<Vec<i32>>> = root_path[root_path.len()-1].borrow();

        assert!( std::ptr::eq(root.deref(), node.borrow().deref() ) );
    }

    
    #[test]
    fn is_goal_test(){
        assert!( ! vec![0].is_goal() );
        assert!( vec![0,1,2,3].is_goal() );
    }
}


/*


impl <'a,T:PartialEq + Copy> PartialEq for  SearchNode<'a,T>{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl <'a,T:PartialEq + Copy> Eq for  SearchNode<'a,T>{
}

impl <'a,T:PartialEq + Copy> Ord for  SearchNode<'a,T>{
    fn cmp(&self, other: &Self) -> Ordering{
        self.level.cmp(&other.level)
    }
}


impl <'a,T:Copy + PartialEq> PartialOrd for SearchNode<'a,T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        //self.level.partial_cmp(&other.level)
        Some(self.cmp(other))
    }
}




impl <'a, T:Copy + Eq + std::hash::Hash> Search<'a,T>{


    pub fn new(root:T, node_expander: &'a NodeExpander<T>, node_checker: &'a NodeChecker<T> ) -> Self{

        
        let mut ret = Search{
            root,
            all_nodes : std::collections::HashMap::new(),
            not_expanded_nodes : std::collections::BTreeSet::new(),
            node_expander,
            node_checker
        };

        let root_node = SearchNode{
             search : &ret,
             to_root : None,
             level : 0,
             data : root
         };

        ret.not_expanded_nodes.insert(O::new(root_node));

        ret
    }





    pub fn next_node_to_expand(&mut self) -> Option<O<SearchNode<'a,T>>> {
        let first = self.not_expanded_nodes.iter().cloned().next();
        match first{
            Some(o) => {
                self.not_expanded_nodes.remove(&o);
                Some(o)
            }
            None => None
        }
    }
    
    pub fn step(&mut self) -> (bool,Option<O<T>>){
        let found_solution = match self.next_node_to_expand(){
            None => None,
            Some(o) => {
                
                if (self.node_checker)(o.borrow().deref()) {
                    Some(o)
                }
                else{
                    let children = (self.node_expander)(o.borrow().deref());
                    
                    children.iter().for_each(|child| {
                        self.not_expanded_nodes.insert( o.borrow().new_child(*child) );
                    });
                    
                    self.all_nodes.insert( node.data, node );
                    
                    None
                }
            }
        };

        (self.not_expanded_nodes.len()>0,found_solution)
    }
}
*/
