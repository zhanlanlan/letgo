/*
 * @lc app=leetcode.cn id=1721 lang=golang
 *
 * [1721] 交换链表中的节点
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func swapNodes(head *ListNode, k int) *ListNode {
	// 本质就是找到正数第k个节点和倒数第k个节点
	// 垃圾题目
	var (
		nth *ListNode
		revNth *ListNode
		dummy = new(ListNode)
	)

	dummy.Next = head

	for i:=1; i<=k; i++ {
		nth = head
		head = head.Next
	}

	revNth = dummy.Next
	for head != nil {
		revNth = revNth.Next
		head = head.Next
	}

	nth.Val, revNth.Val = revNth.Val, nth.Val

	return dummy.Next
}
// @lc code=end

