use std::{
    alloc::{alloc, Layout},
    ptr::{null, null_mut},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[derive(Debug)]
struct X {
    head: *mut Fuck,
    tail: *mut Fuck,
}

impl X {
    unsafe fn new(val: i32) -> Self {
        let f = Fuck::new(val);
        X { head: f, tail: f }
    }

    unsafe fn push_back(&mut self, val: i32) {
        let f = Fuck::new(val);
        (*self.tail).next= f;
        self.tail = f;
    }

    unsafe fn pop_back(&mut self) -> i32 {
        todo!()
    }

    unsafe fn pop_front(&mut self) -> i32 {
        let tmp = self.head;
        self.head = (*self.head).next;
        (*tmp).val
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        unsafe {
            let mut x = X::new(0);
            for i in 1..10 {
                x.push_back(i)
            }

            for i in 1..10 {
                println!("{:?}", x.pop_front());
            }
        }
        
    }
}

#[derive(Debug)]
struct Fuck {
    val: i32,
    next: *mut Fuck,
}

impl Fuck {
    unsafe fn new(val: i32) -> *mut Self {
        let x = alloc(Layout::new::<Fuck>()) as *mut Fuck;
        (*x).val = val;
        x
    }
}
