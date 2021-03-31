use std::fmt::Display;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::cmp::{Ordering, Eq};
use std::cmp::Ordering::*;

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
            Equal => self.cached_state_hash.cmp(&other.cached_state_hash),
//            Equal => simple_hash(&self.state).cmp(&simple_hash(&other.state)),
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


pub fn a_star_search<'a,T:State + PartialEq + Eq + Display>(root:T,search_data : &'a dyn SearchInfo<T>) ->
    (Option<O<SearchNode<'a,T>>>,
     BTreeSet<O<SearchNode<T>>>,
     HashMap<T,O<SearchNode<T>>>)
{
    let mut expanded_counter : usize = 0;
    let root_node = O::new(SearchNode::new_root(root,search_data));

    let mut not_expanded_nodes : BTreeSet<O<SearchNode<T>>> = BTreeSet::new();
    let mut expanded_nodes : HashMap<T,O<SearchNode<T>>> = HashMap::new();
    
    if search_data.is_goal(&root_node.borrow().state) {
        // println!("  root es goal: {}", root_node.borrow().state );
        return (Some(root_node),not_expanded_nodes,expanded_nodes);
    }

    not_expanded_nodes.insert( root_node );

    while let Some(current) = pop(&mut not_expanded_nodes) {
        let state = &current.borrow().state;

        match search_data.max_depth() {
            Some(max) => if current.borrow().level >= max{
                continue;
            }
            None => {
            }
        }

        if let Some(already_expanded) = expanded_nodes.get(state){
            //println!("  se expande de segundas: {}", state );
            if already_expanded.borrow().level > current.borrow().level {
                panic!("El que estaba sin expandir es más corto que el expandido");
            }
            continue;
        }
        
        // println!("Expanding node: {}  heuristic:{}", &current.borrow(), current.borrow().search.heuristic(state) );

        
        assert!( !search_data.is_goal(&state) ); // Se debe detectar antes de meter en not_expanded_nodes
        let children = expand_node(&current);
        expanded_counter = expanded_counter + 1;
        expanded_nodes.insert( state.clone(), current.clone() );
        
        if expanded_counter%1000 == 0{
            println!("Nodos expandidos: {} Nodos sin expandir:{} Ultimo nivel:{}",
                     expanded_nodes.len(), not_expanded_nodes.len(), current.borrow().level );
        }
        

        for child in children{

            //println!("  child: {}", child.borrow() );
            

            // IS GOAL?
            if search_data.is_goal(&child.borrow().state) {
                return (Some(child),not_expanded_nodes,expanded_nodes);
            }

            // HAS BEEN ALREADY EXPANDED?
            if let Some(already_expanded) = expanded_nodes.get(&child.borrow().state){
                if already_expanded.borrow().level > child.borrow().level {
                    panic!("Hay que reenganchar el nodo existente con otro padre, se llegó por un sitio más corto");
                }
            }

            // ALREADY IN NOT EXPANDED NODES?
            // ADD TO not_expanded_nodes
            if let Some(_already_in_not_expanded) = not_expanded_nodes.get(&child){
                // println!("  Ya estaba: {} {}", _already_in_not_expanded.borrow(), child.borrow() );
            }
            else{
                not_expanded_nodes.insert(child);
            }
        }
    }
    
    (None,not_expanded_nodes,expanded_nodes)
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
        let (found,_,_) = a_star_search( Vector(0,0), &SearchToGoal{ goal: Vector(3,4)} );
        assert!( found.is_some() );
        let goal = found.unwrap();
        let path = root_path(&goal);

        for node in path{
            println!("{}", node.borrow() );
        }
    }

}
