
use crate::crossteaser::*;
use crate::search::*;
use crate::ravioli::*;


impl <'a> std::hash::Hash for Board<'a>{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pieces.hash(state);
    }    
}

impl <'a> State for Board<'a>{
}

#[cfg(test)]
mod tests {

    #[test]
    fn rotate() {
    }
}
