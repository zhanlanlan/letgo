/*
 * @lc app=leetcode.cn id=1726 lang=golang
 *
 * [1726] 同积元组
 */

// @lc code=start
func tupleSameProduct(nums []int) int {
	// 用个map可以避免一次循环
	var (
		length = len(nums)
		ma = make(map[int]int, length)
		ans int = 0
	)

	for i, j := range nums {
		for _, n := range nums[i+1:] {
			result := j * n
			if ma[result] != 0 {
				ans += ma[result]
				
			}
			ma[result] ++
		}
	}

	return ans * 8
}
// @lc code=end

