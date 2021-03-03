package main

import (
	"math"
)

/*
 * @lc app=leetcode.cn id=7 lang=golang
 *
 * [7] 整数反转
 */

// @lc code=start
func reverse(x int) int {
	var (
		ans int
	)

	for x != 0 {
		temp := x % 10
		x = x / 10
		if ans > math.MaxInt32/10 || (ans == math.MaxInt32/10 && temp > 7) {
			return 0
		}
		if ans < math.MinInt32/10 || (ans == math.MinInt32/10 && temp < -8) {
			return 0
		}

		ans = ans*10 + temp
	}

	return ans
}

// @lc code=end
