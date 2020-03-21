use std::ops::Deref;

use std::cmp::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Ref;
use std::cell::RefMut;

type OImpl<T> = Rc<RefCell<T>>;

#[derive(Debug)]
pub struct O<T>{
    rc : OImpl<T>
}


impl <T> O<T>{
    pub fn new(data: T) -> Self {
        O{
            rc : Rc::new(RefCell::new(data))
        }
    }

    pub fn borrow(&self) -> Ref<T>{
        self.rc.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T>{
        self.rc.borrow_mut()
    }
}


impl <T:PartialEq> Eq for O<T>{
}

impl <T:PartialEq> PartialEq for  O<T>{
    fn eq(&self, other: &Self) -> bool {
        self.borrow().deref() == other.borrow().deref()
    }
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

impl <T> Clone for O<T>{
    fn clone(&self) -> Self{
        O{
            rc: Rc::clone( &self.rc )
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


    static mut UN_STRUCT_COUNTER : u32 = 0;
    #[test]
    fn un_struct(){
        struct A{
        }

        impl Clone for A{
            fn clone(&self) -> Self{
                unsafe{
                    UN_STRUCT_COUNTER += 1;
                }
                A{}
            }
        }

        let a = A{};
        assert!( unsafe{UN_STRUCT_COUNTER} == 0);

        let b = a.clone();
        assert!( unsafe{UN_STRUCT_COUNTER} == 1);

        let c = b.clone();
        assert!( unsafe{UN_STRUCT_COUNTER} == 2);

        let o1a = O::new(a);
        let o2a = o1a.clone();
        assert!( unsafe{UN_STRUCT_COUNTER} == 2);
    }


}
