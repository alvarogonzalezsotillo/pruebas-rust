use std::cell::Ref;
use std::fmt::{Display, Formatter, Result, Debug};
use std::ops::Deref;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::cmp::{Ordering, Eq};
use std::cmp::Ordering::*;
use std::hash::Hash;
use std::hash::Hasher;

use crate::ravioli::O;

use crate::search::*;

fn simple_hash_fn<T:Hash>(object : &T) -> u64{
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    object.hash(&mut hasher);
    hasher.finish()
}


pub trait Heuristic : Hash + PartialEq + Eq + Display + Debug{
    fn heuristic(&self) -> u64;
    fn simple_hash(&self) -> u64{
        simple_hash_fn(&self)
    }
}

impl <T:State + PartialEq> Eq for SearchNode<T>{
}

impl <T:State + PartialEq> PartialEq for  SearchNode<T>{
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl <T:State + Heuristic> Ord for SearchNode<T>{
    fn cmp(&self, other: &Self) -> Ordering{
        let mine = self.level + self.state.heuristic();
        let their = other.level + other.state.heuristic();
        match mine.cmp(&their){
            Equal => self.state.simple_hash().cmp(&other.state.simple_hash()),
            others => others    
        }
     }
}

impl <T:State + Heuristic> PartialOrd for SearchNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn pop<T:Ord + Clone>( set: &mut BTreeSet<T> ) -> Option<T> {
    let first = {
        set.iter().next().map( |f| f.clone() )
    };
    match first{
        Some(ret) => {
            set.remove(&ret);
            Some(ret)
        },
        None => None
    }
}


pub fn a_star_search<T:State + Heuristic>(root:T) -> Option<O<SearchNode<T>>>
{


    let root_node = O::new(SearchNode::new_root(root));
    if root_node.borrow().state.is_goal() {
        return Some(root_node);
    }

    // https://stackoverflow.com/questions/34028324/how-do-i-use-a-custom-comparator-function-with-btreeset
    // https://stackoverflow.com/questions/35786878/how-can-i-implement-ord-when-the-comparison-depends-on-data-not-part-of-the-comp/35788530#35788530
    let mut not_expanded_nodes : BTreeSet<O<SearchNode<T>>> = BTreeSet::new();
    let mut expanded_nodes : HashMap<T,O<SearchNode<T>>> = HashMap::new();
    not_expanded_nodes.insert( root_node );

    while let Some(current) = pop(&mut not_expanded_nodes) {
        let state = &current.borrow().state;

        println!("Expanding node: {}  heuristic:{}", &current.borrow(), state.heuristic() );
        
        assert!( !state.is_goal() ); // Se debe detectar antes de meter en not_expanded_nodes
        let children = expand_node(&current);
        expanded_nodes.insert( state.clone(), current.clone() );

        for child in children{

            println!("  child: {}", child.borrow() );
            

            // IS GOAL?
            if child.borrow().state.is_goal(){
                return Some(child);
            }

            // HAS BEEN ALREADY EXPANDED?
            if let Some(already_expanded) = expanded_nodes.get(&child.borrow().state){
                if already_expanded.borrow().level > child.borrow().level {
                    panic!("Hay que reenganchar el nodo existente con otro padre, se llegó por un sitio más corto");
                }
            }

            // ALREADY IN NOT EXPANDED NODES?
            // ADD TO not_expanded_nodes
            if let Some(already_in_not_expanded) = not_expanded_nodes.get(&child){
                println!("  Ya estaba: {} {}", already_in_not_expanded.borrow(), child.borrow() );
            }
            else{
                not_expanded_nodes.insert(child);
            }
        }
    }
    
    None
}


#[cfg(test)]
mod tests{
    use crate::search::*;
    use crate::search::astar::*;
    use std::fmt::*;

    static GOAL : (u64,u64) = (3,4);

    #[derive(Clone,Hash,Debug,Eq,PartialEq)]
    struct Vector(u64,u64);

    impl Display for Vector {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }
    
    impl State for Vector{
        fn expand_state(&self) -> Vec<Self> {
            vec![
                Vector(self.0+1,self.1  ),
                Vector(self.0  ,self.1+1)
            ]
        }
        fn is_goal(&self) -> bool {
            self.0 == 3 && self.1 == 4
        }
    }

    impl Heuristic for Vector{
        fn heuristic(&self) -> u64{
            let dx = self.0 as i64 - GOAL.0 as i64;
            let dy = self.1 as i64 - GOAL.1 as i64;
            let sqr = (dx*dx + dy*dy) as f64;
            sqr.sqrt() as u64
        }
    }

    
    #[test]
    fn a_star_test(){
        let found = a_star_search( Vector(0,0) );
        assert!( found.is_some() );

        let path = root_path(&found.unwrap());

        for node in path{
            println!("{}", node.borrow() );
        }
    }
    
}
