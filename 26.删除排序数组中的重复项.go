/*
 * @lc app=leetcode.cn id=26 lang=golang
 *
 * [26] 删除排序数组中的重复项
 */

// @lc code=start
func removeDuplicates(nums []int) int {
	var (
		pre int
		currPoint int
		length = len(nums)
	)

	if length <= 1 {
		return length
	}	

	pre = nums[0]
	currPoint = 1
	for i := 1; i < length; i++ {
		if nums[i] == pre {
			continue
		}
		pre = nums[i]
		nums[currPoint] = nums[i]
		currPoint ++
	}

	return currPoint
}
// @lc code=end

