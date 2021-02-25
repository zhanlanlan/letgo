package main

import "sort"

func fourSum(nums []int, target int) [][]int {
	length := len(nums)
	sort.Ints(nums)
	ans := make([][]int, 0)

	for i, j := range nums {
		if i > 0 && nums[i-1] == j {
			continue
		}
		for b := i+1; b<length; b++ {
			if b > i+1 && nums[b-1] == nums[b] {
				continue
			}

			c := b+1
			d := length -1
			for c < d {
				if c > b+1 && nums[c] == nums[c-1] {
					c++
					continue
				}

				sum := j + nums[b] +nums[c] + nums[d]
				if sum == target{
					ans = append(ans, []int{j, nums[b], nums[c], nums[d]})
					c++
					d--
				} else if sum < target {
					c++
				} else {
					d--
				}
			}
			
		}
		
	}

	return ans
}