// https://leetcode-cn.com/problems/find-first-and-last-position-of-element-in-sorted-array/

package main

func searchRange(nums []int, target int) []int {
	length := len(nums)
	left, right := 0, length-1

	var mid int
	// 一次二分查找
	for left <= right {
		mid := (left + right) / 2 
		if 
	}

	// 另一次左右扩散

	
}