package main

/*
 * @lc app=leetcode.cn id=50 lang=golang
 *
 * [50] Pow(x, n)
 */

// @lc code=start
func myPow(x float64, n int) float64 {
	var result float64

	if n >= 0 {
		result = myPowInternal(x, n)
	} else {
		result = myPowInternal(x, -n)
		result = 1 / result
	}

	return result
}

func myPowInternal(x float64, n int) float64 {
	// 递归超内存
	// 迭代超时间
	// 那就二分了
	if n == 0 {
		return 1
	}

	if n == 1 {
		return x
	}

	mid := n / 2
	result := myPowInternal(x, mid)
	return result * result * myPowInternal(x, n-(mid*2))

}

// @lc code=end
