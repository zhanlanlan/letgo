use std::cell::RefCell;
use std::rc::Rc;

use crate::common::tree::Tree;

/// 题目来自 [OI Wiki](https://oi-wiki.org/basic/divide-and-conquer/)
fn path_sum_iii(root: &Option<Rc<RefCell<Tree>>>, target: i32) -> i32 {
    if let Some(n) = root {
        return xxx(&root, target)
            + path_sum_iii(&n.borrow().left, target)
            + path_sum_iii(&n.borrow().right, target);
    }
    return 0;
}

fn xxx(node: &Option<Rc<RefCell<Tree>>>, value: i32) -> i32 {
    let mut shooot = 0;
    let left_shoot;
    let right_shoot;
    if let Some(n) = node {
        if n.borrow().val == value {
            shooot = 1;
        }

        left_shoot = xxx(&n.borrow().left, value - n.borrow().val);
        right_shoot = xxx(&n.borrow().right, value - n.borrow().val);

        return shooot + left_shoot + right_shoot;
    }

    return 0;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_path_sum_iii() {}
}
