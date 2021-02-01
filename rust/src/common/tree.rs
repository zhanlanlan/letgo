use std::cell::RefCell;
use std::rc::Rc;

use std::alloc::{alloc, Layout};

pub struct Tree {
    pub val: i32,
    pub left: Option<Rc<RefCell<Tree>>>,
    pub right: Option<Rc<RefCell<Tree>>>,
}

impl Tree {
    pub fn new(val: i32) -> Tree {
        Tree {
            val,
            left: None,
            right: None,
        }
    }
}

type TreeNode = Option<Rc<RefCell<BaseTree>>>;

pub struct BaseTree {
    pub val: i32,
    pub left: TreeNode,
    pub right: TreeNode,
}

fn new_tree_node(val: i32) -> TreeNode {
    Some(Rc::new(RefCell::new(BaseTree {
        val,
        left: None,
        right: None,
    })))
}

#[derive(Debug)]
struct Tee {
    val: i32,
    left: Option<*mut Tee>,
    right: Option<*mut Tee>,
}

impl Tee {
    fn new(val: i32) -> Option<*mut Tee> {
        let te = unsafe {
            let te = alloc(Layout::new::<Tee>()) as *mut Tee;
            (*te).val = val;
            (*te).left = None;
            (*te).right = None;
            te
        };
        Some(te)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_xxx() {
        let tee = Tee::new(123);

        if let Some(x) = tee {
            unsafe {
                println!("{:?}", *x);
            }
        }
    }
}
