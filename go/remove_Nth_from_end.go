package main

func removeNthFromEnd(head *ListNode, n int) *ListNode {
	var (
		nth   *ListNode
		dummy = new(ListNode)
	)
	dummy.Next = head
	nth = dummy

	for i := 0; i < n; i++ {
		if head != nil {
			head = head.Next
		} else {
			return dummy.Next
		}
	}

	for head != nil {
		head = head.Next
		nth = nth.Next
	}

	nth.Next = nth.Next.Next

	return dummy.Next
}
