use std::ops::Deref;

use std::cmp::*;


struct O<T>{
    rc : std::rc::Rc<std::cell::RefCell<T>>
}


impl <T> O<T>{
    pub fn new(data: T) -> Self {
        O{
            rc : std::rc::Rc::new(std::cell::RefCell::new(data))
        }
    }

    pub fn borrow(&self) -> std::cell::Ref<T>{
        self.rc.borrow()
    }

    pub fn borrow_mut(&self) -> std::cell::RefMut<T>{
        self.rc.borrow_mut()
    }

}


impl <T:PartialEq> Eq for O<T>{
}

impl <T:PartialEq + Ord> PartialOrd for O<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl <T:Ord> Ord for O<T>{
    fn cmp(&self, other: &Self) -> Ordering{
        self.borrow().cmp(&other.borrow())
    }
}



impl <T:PartialEq> PartialEq for  O<T>{
    fn eq(&self, other: &Self) -> bool {
        self.borrow().deref() == other.borrow().deref()
    }
}

impl <T> Clone for O<T>{
    fn clone(&self) -> Self{
        O{
            rc: std::rc::Rc::clone( &self.rc )
        }
   } 
}


#[cfg(test)]
mod tests {

    use crate::ravioli::*;
    
    #[test]
    fn simple(){
        let o = O::new(1);
        assert!( *o.borrow() == 1 );
    }

    #[test]
    fn clone(){
        let o1 : O<i8> = O::new(1);
        let o2 : O<i8> = o1.clone();

        *o2.borrow_mut() = 2;

        assert!( *o1.borrow() == 2 );
    }


}
