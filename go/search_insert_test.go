package main

import (
	"log"
	"testing"
)

func TestSearchInsert(t *testing.T){
	nums := []int {1,3,5,6}

	target := 5
	log.Println(searchInsert(nums, target))

	target = 2
	log.Println(searchInsert(nums, target))

	target = 7
	log.Println(searchInsert(nums, target))

	target = 0
	log.Println(searchInsert(nums, target))

}