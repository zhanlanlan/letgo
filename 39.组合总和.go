package main

import (
	"sort"
)

/*
 * @lc app=leetcode.cn id=39 lang=golang
 *
 * [39] 组合总和
 */

// @lc code=start
func combinationSum(candidates []int, target int) [][]int {
	sort.Ints(candidates)

	return combinationSumInternal(candidates, target)
}

func combinationSumInternal(candidates []int, target int) [][]int {

	ans := make([][]int, 0)

	if len(candidates) == 0 {
		return ans
	}

	if target < candidates[0] {
		return ans
	}

	for i, j := range candidates {
		if target == j {
			ans = append(ans, []int{j})
			return ans
		}
		cbs := combinationSumInternal(candidates[i:], target-j)
		for _, t := range cbs {
			x := []int{j}
			x = append(x, t...)
			ans = append(ans, x)
		}
	}

	return ans

}

// @lc code=end
