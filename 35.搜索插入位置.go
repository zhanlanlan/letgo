/*
 * @lc app=leetcode.cn id=35 lang=golang
 *
 * [35] 搜索插入位置
 */

// @lc code=start
func searchInsert(nums []int, target int) int {
	var (
		length = len(nums)
		left = 0
		right = length - 1
		mid = 0
	)

	for left <= right {
		mid = (left + right) / 2 
		if nums[mid] == target {
			return mid
		}
		if nums[mid] > target {
			right = mid -1
		} else {
			left = mid + 1
		}
	}
	
	if nums[mid] > target {
		return mid
	} else {
		return mid + 1
	}

}
// @lc code=end

