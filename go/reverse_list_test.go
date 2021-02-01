package main

import "testing"

func TestReverseList(t *testing.T) {
	var root *ListNode

	root = PushListNode(root, 100)
	root = PushListNode(root, 200)
	root = PushListNode(root, 300)
	root = PushListNode(root, 400)
	root = PushListNode(root, 500)
	root = PushListNode(root, 600)
	root = PushListNode(root, 700)
	root = PushListNode(root, 800)
	root = PushListNode(root, 900)

	PrintListNode(root)

	root = reverseList(root)

	PrintListNode(root)
}
