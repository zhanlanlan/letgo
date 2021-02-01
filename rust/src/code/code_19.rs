use common::ListNode;

use crate::common;

fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_nth_from_end() {
        let mut head = Some(Box::from(ListNode::new(1)));
        let mut tail = head.as_mut().unwrap();

        for i in 2..10 {
            tail.next = Some(Box::new(ListNode::new(i)));
            tail = tail.next.as_mut().unwrap();
        }

        let n  = 2;

        println!("{:?}", remove_nth_from_end(head, n));
    }
}
