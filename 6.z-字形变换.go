package main

/*
 * @lc app=leetcode.cn id=6 lang=golang
 *
 * [6] Z 字形变换
 */

// @lc code=start
func convert(s string, numRows int) string {

	if numRows == 1 {
		return s
	}

	var (
		rows = make([][]rune, numRows)
		flag = -1
		currRow = 0
		ans []rune
	)

	for _, j := range s {
		rows[currRow] = append(rows[currRow], j)
		if currRow == 0 || currRow == numRows-1 {
			flag = -flag
		}
		currRow += flag
	}

	for _, j := range rows {
		ans = append(ans, j...)
	}

	return string(ans)
}
// @lc code=end

