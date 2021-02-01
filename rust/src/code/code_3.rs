use std::collections::HashMap;

fn length_of_longest_substring(s: String) -> i32 {
    let mut pos = HashMap::<u8, usize>::new();
    let mut start = 0;
    let mut max_len = 0;
    let mut cur_len = 0;

    for (i, j) in s.as_bytes().iter().enumerate() {
        if let Some(idx) = pos.get(j) {
            if *idx < start {
                cur_len += 1;
            } else {
                cur_len = cur_len - (*idx  - start);
                start = *idx + 1;
            }
        } else {
            cur_len += 1;
        }

        pos.insert(*j, i);

        max_len = max_len.max(cur_len);
    }

    max_len as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        let s = String::from("pwwkew");

        println!("{:?}", length_of_longest_substring(s));
    }
}
