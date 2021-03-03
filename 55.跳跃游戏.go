package main

/*
 * @lc app=leetcode.cn id=55 lang=golang
 *
 * [55] 跳跃游戏
 */

// @lc code=start
func canJump(nums []int) bool {
	var (
		furthest int
	)

	// if len(nums) <= 1 {
	// 	return true
	// }

	for i, j := range nums {
		if furthest >= i {
			furthest = canJumpMax(furthest, i+j)
		} else {
			return false
		}
	}

	return furthest >= len(nums)-1
}

func canJumpMax(a, b int) int {
	if a > b {
		return a
	}
	return b
}

// @lc code=end
