// https://leetcode-cn.com/problems/search-in-rotated-sorted-array/

package main

func search(nums []int, target int) int {
	length := len(nums)
	left, right := 0, length-1
	for left <= right {
		mid := (left + right) / 2

		if target == nums[mid] {
			return mid
		}

		// 如果右边是有序的
		if nums[mid] <= nums[right] {
			if target >= nums[mid] && target <= nums[right] {
				// 且在右边
				left = mid + 1
				continue
			} else {
				// 不在右边
				right = mid - 1
				continue
			}
		} else {
			// 那么肯定左边是有序的咯
			if target >= nums[left] && target <= nums[mid] {
				right = mid - 1
				continue
			} else {
				left = mid + 1
			}
		}

	}

	return -1

}
