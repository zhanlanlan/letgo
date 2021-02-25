package main

import "strings"


func longestCommonPrefix(strs []string) string {
	if len(strs) < 1 {
		return ""
	}


	ans := strs[0]


	for ans != "" {
		for _,v := range strs[1:] {
			if !strings.HasPrefix(v, ans) {
				goto c
			}
		}
		return ans

		c:
		ans = ans[:len(ans)-1]
	}

	return ans
}