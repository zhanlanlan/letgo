package main

func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	var carry int
	var head, dummy *ListNode
	dummy = new(ListNode)
	head = dummy

	for l1 != nil && l2 != nil {
		head.Next = &ListNode{
			Val: ((l1.Val + l2.Val + carry) % 10),
		}
		if l1.Val+l2.Val + carry >= 10 {
			carry = 1
		} else {
			carry = 0
		}

		head = head.Next
		l1 = l1.Next
		l2 = l2.Next
	}

	if l1 != nil {
		head.Next = l1
	} else {
		head.Next = l2
	}

	for carry != 0 {
		if head.Next == nil {
			head.Next = &ListNode{
				Val: carry,
			}
			carry = 0
		} else {
			head.Next.Val = head.Next.Val + carry
			if head.Next.Val >= 10 {
				head.Next.Val = head.Next.Val % 10
				carry = 1
				head = head.Next
			} else {
				carry = 0
			}
		}
	}

	return dummy.Next
}
