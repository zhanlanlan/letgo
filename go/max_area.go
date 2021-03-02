package main

func maxArea(height []int) int {

	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}

	var (
		left  = 0
		l     = len(height)
		right = l - 1

		ans int
	)

	for right-left > 0 {
		ans = max(ans, min(height[left], height[right])*(right-left))

		if height[left] < height[right] {
			left++
		} else {
			right--
		}

	}

	return ans
}
