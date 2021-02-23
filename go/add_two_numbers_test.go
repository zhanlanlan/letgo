package main

import (
	"log"
	"testing"
)

func TestAddTwoNumbers(t *testing.T) {
	var l1, l2 *ListNode

	l1 = PushListNode(l1, 3)
	l1 = PushListNode(l1, 7)


	l2 = PushListNode(l2, 9)
	l2 = PushListNode(l2, 2)


	log.Println("l1")
	PrintListNode(l1)

	log.Println("l2")
	PrintListNode(l2)

	res := addTwoNumbers(l1, l2)

	log.Println("res")
	PrintListNode(res)


}