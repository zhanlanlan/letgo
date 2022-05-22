// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

// An input string is valid if:

// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.

// Example 1:
// Input: s = "()"
// Output: true
//
// Example 2:
// Input: s = "()[]{}"
// Output: true
//
// Example 3:
// Input: s = "(]"
// Output: false

pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];

    for i in s.into_bytes() {
        match i {
            b'(' | b'{' | b'[' => stack.push(i),

            b')' => {
                if Some(b'(') != stack.pop() {
                    return false;
                }
            }

            b'}' => {
                if Some(b'{') != stack.pop() {
                    return false;
                }
            }

            b']' => {
                if Some(b'[') != stack.pop() {
                    return false;
                }
            }

            _ => return false,
        }
    }

    return stack.is_empty();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid("()".into()), true);
        assert_eq!(is_valid("()[]{}".into()), true);
        assert_eq!(is_valid("(]".into()), false);
    }
}
