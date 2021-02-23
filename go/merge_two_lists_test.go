package main

import "testing"

func TestMergeTwoLists(t *testing.T) {
	var root1 *ListNode
	var root2 *ListNode

	root1 = PushListNode(root1, 1)
	root1 = PushListNode(root1, 2)
	root1 = PushListNode(root1, 4)
	// root1 = PushListNode(root1, 7)

	root2 = PushListNode(root2, 1)
	root2 = PushListNode(root2, 3)
	root2 = PushListNode(root2, 4)
	// root2 = PushListNode(root2, 8)

	newRoot := mergeTwoLists(root1, root2)

	PrintListNode(newRoot)
}