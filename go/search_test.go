package main

import (
	"log"
	"testing"
)

func TestSearch(t *testing.T) {
	nums := []int{1}
	target := 0

	log.Println(search(nums, target))
}
