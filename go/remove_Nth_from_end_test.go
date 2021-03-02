package main

import "testing"

func TestRemoveNthFromEnd(t *testing.T) {
	var root *ListNode

	root = PushListNode(root, 100)

	PrintListNode(root)

	root = removeNthFromEnd(root, 1)
	PrintListNode(root)
}
