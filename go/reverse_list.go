package main

func reverseList(head *ListNode) *ListNode {
	var (
		newHead *ListNode
		next    = head
	)

	for next != nil {
		tmp := next.Next
		next.Next = newHead
		newHead = next
		next = tmp
	}

	return newHead

}
