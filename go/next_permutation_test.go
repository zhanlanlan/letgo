package main

import (
	"log"
	"testing"
)

func TestNextPermutation(t *testing.T) {
	nums := []int{1,2,3}

	nextPermutation(nums)

	log.Println(nums)
}