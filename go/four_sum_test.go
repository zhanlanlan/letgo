package main

import (
	"log"
	"testing"
)

func TestFourSum(t *testing.T) {
	nums := []int{1, 0, -1, 0, -2, 2}
	target := 0

	log.Println(fourSum(nums, target))
}
