pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut result = Vec::new();

    if let Some((first, rest)) = strs.split_first() {
        for (i, c) in first.bytes().enumerate() {
            if rest.iter().all(|s| s.as_bytes().get(i) == Some(&c)) {
                result.push(c);
            } else {
                break;
            }
        }
    }

    String::from_utf8(result).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            longest_common_prefix(vec!["flower".into(), "flow".into(), "flight".into()]),
            "fl"
        );

        assert_eq!(
            longest_common_prefix(vec!["dog".into(), "racecar".into(), "car".to_string()]),
            ""
        );
    }
}
