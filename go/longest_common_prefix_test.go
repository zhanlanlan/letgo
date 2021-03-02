package main

import (
	"log"
	"testing"
)

func TestLongestCommonPrefix(t *testing.T) {
	a := []string{"flower","flow","flight"}

	log.Println(longestCommonPrefix(a))
}