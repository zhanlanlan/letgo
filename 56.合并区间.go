package main

import (
	"sort"
)

/*
 * @lc app=leetcode.cn id=56 lang=golang
 *
 * [56] 合并区间
 */

// @lc code=start
func merge(intervals [][]int) [][]int {
	if len(intervals) < 2 {
		return intervals
	}

	sort.Sort(SortByFirstElem(intervals))
	var (
		ans     = make([][]int, 0)
		currlen = 0
	)
	ans = append(ans, intervals[0])

	for i := 1; i < len(intervals); i++ {
		if ans[currlen][1] >= intervals[i][0] {

			ans[currlen][1] = mergeMax(ans[currlen][1], intervals[i][1])
			continue
		}
		ans = append(ans, intervals[i])
		currlen++
	}

	return ans
}

type SortByFirstElem [][]int

func (a SortByFirstElem) Len() int      { return len(a) }
func (a SortByFirstElem) Swap(i, j int) { a[i], a[j] = a[j], a[i] }
func (a SortByFirstElem) Less(i, j int) bool {
	if a[i][0] < a[j][0] {
		return true
	} else if a[i][0] == a[j][0] {
		if a[i][1] < a[j][1] {
			return true
		} else {
			return false
		}
	} else {
		return false
	}
}

func mergeMax(a, b int) int {
	if a > b {
		return a
	}
	return b
}

// @lc code=end
