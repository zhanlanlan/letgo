// https://leetcode-cn.com/problems/swap-nodes-in-pairs/
package main

func swapPairs(head *ListNode) *ListNode {
	dummy := new(ListNode)
	dummy.Next = head
	swapPairsT(dummy)

	return dummy.Next
}

func swapPairsT(dummy *ListNode) {
	if dummy.Next == nil || dummy.Next.Next == nil {
		return
	}

	// log.Printf("before: %d, %d", dummy.Next.Val, dummy.Next.Next.Val)

	// 这段代码真是超耐磨
	temp := dummy.Next
	dummy.Next = dummy.Next.Next
	temp.Next = dummy.Next.Next
	dummy.Next.Next = temp

	// log.Printf("after: %d, %d", dummy.Next.Val, dummy.Next.Next.Val)


	swapPairsT(dummy.Next.Next)
}
