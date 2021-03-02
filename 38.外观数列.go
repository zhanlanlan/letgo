package main

import (
	"strconv"
	"strings"
)

/*
 * @lc app=leetcode.cn id=38 lang=golang
 *
 * [38] 外观数列
 */

// @lc code=start
func countAndSay(n int) string {
	var (
		buf strings.Builder
		s   string
	)

	if n == 1 {
		return "1"
	}

	s = countAndSay(n - 1)

	var c rune = 0
	cnt := 0
	for _, v := range s {
		if c == v {
			cnt++
			continue
		}

		if c != 0 {
			buf.WriteString(strconv.Itoa(cnt))
			buf.WriteRune(c)
		}

		c = v
		cnt = 1

	}

	buf.WriteString(strconv.Itoa(cnt))
	buf.WriteRune(c)

	return buf.String()
}
// @lc code=end
