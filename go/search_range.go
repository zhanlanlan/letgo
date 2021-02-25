// https://leetcode-cn.com/problems/find-first-and-last-position-of-element-in-sorted-array/

package main

func searchRange(nums []int, target int) []int {

	if len(nums) == 0{
		return []int{-1, -1}
	}

	length := len(nums)
	left, right := 0, length-1

	var mid int
	// 一次二分查找
	for left <= right {
		mid = (left + right) / 2 
		if nums[mid] == target {
			break
		}

		if nums[mid] > target {
			right = mid -1
		} else {
			left = mid +1
		}
	}

	if nums[mid] != target {
		return []int{-1, -1}
	}

	// 另一次左右扩散
	// 确定左边
	left = mid
	for left>0&&nums[left-1] == target {
		left--
	}

	right = mid
	for  right<length-1&&nums[right+1] ==target{
		right ++
	} 

	return []int{left, right}
	
}