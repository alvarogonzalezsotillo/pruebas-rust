use std::cmp::Ordering;

use crate::ravioli::O;


struct SearchNode<T:State>{
    to_root : O<SearchNode<T>>,
    level : u16,
    state : T
}

trait State : Clone{
    fn expand_state(&self) -> Vec<Self>;
    fn is_goal(&self) -> bool;
}


struct Search<T:State>{
    root : T,
    all_nodes : std::collections::HashMap<T,O<SearchNode<T>>>,
    not_expanded_nodes : std::collections::BTreeSet<O<SearchNode<T>>>,
}

#[cfg(test)]
mod tests{

    use crate::search::*;
    
    #[test]
    fn viability(){
        impl State for Vec<i32>{
            fn expand_state(&self) -> Vec<Vec<i32>> {
                [1,2].
                    iter().
                    map( |i| {
                        let mut child = self.clone();
                        child.push(*i);
                        child
                    }).
                    collect()
            }

            fn is_goal(&self) -> bool {
                self.len() == 3
            }
        }
    }
}


/*
pub fn new_child(node : O<SearchNode<T>>, new_child: T) -> O<SearchNode<T>>{
    SearchNode{
        search : self.search.clone(),
        to_root: Some(&self),
        level : self.level+1,
        data: new_child
    }
}


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
