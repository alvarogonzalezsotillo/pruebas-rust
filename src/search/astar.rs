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

impl <'a,T:State + PartialEq> Eq for SearchNode<'a,T>{
}

impl <'a,T:State + PartialEq> PartialEq for  SearchNode<'a,T>{
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl <'a,T:State + PartialEq> Ord for SearchNode<'a,T>{
    fn cmp(&self, other: &Self) -> Ordering{
        let search_data = self.search;
        let mine = self.level + search_data.heuristic(&self.state);
        let their = other.level + search_data.heuristic(&other.state);
        match mine.cmp(&their){
            Equal => simple_hash(&self.state).cmp(&simple_hash(&other.state)),
            others => others    
        }
    }
}

impl <'a,T:State + PartialEq> PartialOrd for SearchNode<'a,T> {
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


pub fn a_star_search<'a,T:State + PartialEq + Eq + Display>(root:T,search_data : &'a dyn SearchInfo<T>) -> Option<O<SearchNode<'a,T>>>
{


    let root_node = O::new(SearchNode::new_root(root,search_data));
    
    if search_data.is_goal(&root_node.borrow().state) {
        return Some(root_node);
    }

    // https://stackoverflow.com/questions/34028324/how-do-i-use-a-custom-comparator-function-with-btreeset
    // https://stackoverflow.com/questions/35786878/how-can-i-implement-ord-when-the-comparison-depends-on-data-not-part-of-the-comp/35788530#35788530
    let mut not_expanded_nodes : BTreeSet<O<SearchNode<T>>> = BTreeSet::new();
    let mut expanded_nodes : HashMap<T,O<SearchNode<T>>> = HashMap::new();
    not_expanded_nodes.insert( root_node );

    while let Some(current) = pop(&mut not_expanded_nodes) {
        let state = &current.borrow().state;

        println!("Expanding node: {}  heuristic:{}", &current.borrow(), current.borrow().search.heuristic(state) );

        
        assert!( !search_data.is_goal(&state) ); // Se debe detectar antes de meter en not_expanded_nodes
        let children = expand_node(&current);
        expanded_nodes.insert( state.clone(), current.clone() );

        for child in children{

            println!("  child: {}", child.borrow() );
            

            // IS GOAL?
            if search_data.is_goal(&child.borrow().state) {
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


    #[derive(Clone,Hash,Debug,Eq,PartialEq)]
    struct Vector(u64,u64);

    impl Display for Vector {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }
    
    impl State for Vector{
    }

    


    #[derive(Debug)]
    struct SearchToGoal{
        goal: Vector
    }

    impl SearchInfo<Vector> for SearchToGoal{
        fn heuristic(&self,state: &Vector) -> u64{
            println!("Heuristica para {} con objetivo {}", state, self.goal );
            let dx = state.0 as i64 - self.goal.0 as i64;
            let dy = state.1 as i64 - self.goal.1 as i64;
            let sqr = (dx*dx + dy*dy) as f64;
            sqr.sqrt() as u64
        }

        fn expand_state(&self,state: &Vector) -> Vec<Vector> {
            vec![
                Vector(state.0+1,state.1  ),
                Vector(state.0  ,state.1+1)
            ]
        }
        fn is_goal(&self,state: &Vector) -> bool {
            state.0 == self.goal.0 && state.1 == self.goal.1
        }
        
    }


    #[test]
    fn a_star_test(){
        let found = a_star_search( Vector(0,0), &SearchToGoal{ goal: Vector(3,4)} );
        assert!( found.is_some() );
        let goal = found.unwrap();
        let path = root_path(&goal);

        for node in path{
            println!("{}", node.borrow() );
        }
    }

}
