fn valid_braces(s: &str) -> bool {
    let mut stack = vec![];

    for i in s.chars() {
        match i {
            '{' | '[' | '(' => stack.push(i),
            '}' => {
                if stack.pop().unwrap_or('x') != '{' {
                    return false;
                }
            }
            ']' => {
                if stack.pop().unwrap_or('x') != '[' {
                    return false;
                }
            }
            ')' => {
                if stack.pop().unwrap_or('x') != '(' {
                    return false;
                }
            }
            _ => return false,
        }
    }

    if stack.len() == 0 {
        return true;
    }

    false
}

#[test]
fn basic_tests() {
    assert_eq!(valid_braces("()"), true);
    assert_eq!(valid_braces("[(])"), false);
}
