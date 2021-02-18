package main

import (
	"log"
	"testing"
)

func TestLengthOfLongestSubstring(t *testing.T) {
	s := "abcabcbb"
	res := lengthOfLongestSubstring(s)

	log.Println(res)
}