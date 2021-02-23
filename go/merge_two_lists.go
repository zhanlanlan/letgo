package main

func mergeTwoLists(l1 *ListNode, l2 *ListNode) *ListNode {
	var dummy = new(ListNode)
	var tail = dummy

	for l1 != nil || l2 != nil {
		if l1 == nil {
			tail.Next = l2
			tail = tail.Next
			l2 = l2.Next
			continue
		}

		if l2 == nil {
			tail.Next = l1
			tail = tail.Next
			l1 = l1.Next
			continue
		}

		if l1.Val < l2.Val {
			tail.Next = l1
			tail = tail.Next
			l1 = l1.Next
		} else {
			tail.Next = l2
			tail = tail.Next
			l2 = l2.Next
		}
	}

	return dummy.Next
}