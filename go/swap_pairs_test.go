package main

import (
	"testing"
)

func TestSwapPairs(t *testing.T) {
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
	
	// dummy := new(ListNode)
	// dummy.Next = root

	// temp := dummy.Next
	// dummy.Next = dummy.Next.Next
	// temp.Next = dummy.Next.Next
	// dummy.Next.Next = temp

	// PrintListNode(dummy.Next)

	PrintListNode(root)

	root = swapPairs(root)
	PrintListNode(root)
}