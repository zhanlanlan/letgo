package main

import (
	"log"
	"testing"
)

func TestSearchRange(t *testing.T) {
	nums := []int{5, 7, 7, 8, 8, 10}
	target := 8

	log.Println(searchRange(nums, target))
}
