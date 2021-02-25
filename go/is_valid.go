package main

func isValid(s string) bool {
	stack := make([]rune, 0)
	for _, v := range s {
		if v == '(' || v == '{' || v == '[' {
			stack = append(stack, v)
		} else {
			if len(stack) == 0 {
				return false
			}
			c := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			if v == ')' {
				if c != '(' {
					return false
				}
			} else if v == '}' {
				if c != '{' {
					return false
				}
			} else if v == ']' {
				if c != '[' {
					return false
				}
			}
		}
	}

	return len(stack) == 0
}
