use std::cmp::Ordering;


struct SearchNode<'a,T:Copy>{
    search : &'a Search<'a,T>,
    to_root : Option<&'a SearchNode<'a,T>>,
    level : u16,
    data : T
}

impl <'a,T:Copy> SearchNode<'a,T>{
    pub fn new_child(&'a self, new_child: T) -> SearchNode<'a,T>{
        SearchNode{
            search : self.search,
            to_root: Some(&self),
            level : self.level+1,
            data: new_child
        }
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


type NodeExpander<T> = dyn Fn(&T) -> Vec<T>;
type NodeChecker<T> = dyn Fn(&T) -> bool;



struct Search<'a,T:Copy>{
    root : T,
    all_nodes : std::collections::HashMap<T,O<SearchNode<'a,T>>>,
    not_expanded_nodes : std::collections::BTreeSet<O<SearchNode<'a,T>>>,
    node_expander : &'a NodeExpander<T>,
    node_checker : &'a NodeChecker<T>,
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
