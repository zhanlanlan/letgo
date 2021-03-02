package main

import "testing"

func TestDeleteDuplicates(t *testing.T) {
	var root *ListNode

	root = PushListNode(root, 100)
	root = PushListNode(root, 100)
	root = PushListNode(root, 300)
	root = PushListNode(root, 300)
	root = PushListNode(root, 500)
	root = PushListNode(root, 500)
	root = PushListNode(root, 700)
	root = PushListNode(root, 700)
	root = PushListNode(root, 900)

	PrintListNode(root)

	root = deleteDuplicates(root)
	PrintListNode(root)
}