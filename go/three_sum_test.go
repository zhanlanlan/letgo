package main

import (
	"log"
	"testing"
)

func TestThreeSum(t *testing.T) {
	nums := []int{-1,0,1,2,-1,-4}

	log.Println(threeSum(nums))
}