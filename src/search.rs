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

#[cfg(test)]
mod tests{

    use crate::search::*;

    impl State for Vec<i32>{
        fn expand_state(&self) -> Vec<Vec<i32>> {

            if self.len() < 4 {
                
                [0,1,2].
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
    fn expand(){
        let vec = vec![0];
        let children = vec.expand_state();
        println!("{:?}", children );
        assert!(children == vec![ vec![0,0], vec![0,1], vec![0,2] ]);
    }

    #[test]
    fn expand_node(){
        let vec = vec![0];
        let node = O::new(SearchNode::new_root(vec.clone()));
        let children = crate::search::expand_node(&node);

        println!("{}", children.iter().map(|c| c.to_string() ).collect::<Vec<String>>().join(" ") );
        assert!( children.len() == 3 );

        let children = crate::search::expand_node(&children[0]);
        println!("{}", children.iter().map(|c| c.to_string() ).collect::<Vec<String>>().join(" ") );
        assert!( children.len() == 3 );
    }
    

    #[test]
    fn root_path(){
        let vec = vec![0];
        let node = O::new(SearchNode::new_root(vec));
        let children = crate::search::expand_node(&node);
        let children = crate::search::expand_node(&children[0]);
        let children = crate::search::expand_node(&children[0]);

        let root_path = crate::search::root_path(&children[0]);
        
        println!("{}", root_path.iter().map(|c| c.to_string() ).collect::<Vec<String>>().join(" ") );
        assert!( root_path.len() == 4 );

        let root : Ref<SearchNode<Vec<i32>>> = root_path[root_path.len()-1].borrow();

        assert!( std::ptr::eq(root.deref(), node.borrow().deref() ) );
    }

    
    #[test]
    fn is_goal(){
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
