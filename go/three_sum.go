package main

import "sort"

func threeSum(nums []int) [][]int {
	length := len(nums)
	sort.Ints(nums)
	ans := make([][]int, 0)

	// 枚举 a
	for a, v := range nums {
		// 需要和上一次枚举的数不相同
		if a > 0 && v == nums[a-1] {
			continue
		}
		b := a + 1
		c := length - 1
		for b < c && b < length {
			if b > a+1 && nums[b] == nums[b-1] {
				b++
				continue
			}
			sum := nums[a] + nums[b] + nums[c]
			if sum == 0 {
				ans = append(ans, []int{nums[a], nums[b], nums[c]})
				b++
				c--
			} else if sum < 0 {
				b++
			} else {
				c--
			}

		}
	}
	return ans
}
