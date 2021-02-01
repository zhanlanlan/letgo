package main

import "fmt"

// ListNode ...
type ListNode struct {
	Val  int
	Next *ListNode
}

// PushListNode ...
func PushListNode(root *ListNode, val int) *ListNode {
	if root == nil {
		root = new(ListNode)
		root.Val = val
		return root
	}

	pushListNode(root, val)

	return root

}

func pushListNode(root *ListNode, val int) {
	if root.Next == nil {
		root.Next = new(ListNode)
		root.Next.Val = val
		return
	}

	pushListNode(root.Next, val)
}

// PrintListNode ...
func PrintListNode(root *ListNode) {
	if root != nil {
		fmt.Print(root.Val)
		if root.Next != nil {
			fmt.Print("->")
		}
		PrintListNode(root.Next)
	} else {
		fmt.Println()
	}
}
