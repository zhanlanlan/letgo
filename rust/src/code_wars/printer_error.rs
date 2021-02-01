fn printer_error(s: &str) -> String {
    let (err, cnt) = s.chars().fold((0, 0), |acc, c| {
        let cnt = acc.1 + 1;
        let mut err = acc.0;
        if c > 'm' || c < 'a' {
            err += 1;
        }

        (err, cnt)
    });

    format!("{}/{}", err, cnt)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_pass_all_the_tests_provided() {
        assert_eq!(
            &printer_error("aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"),
            "3/56"
        );
        assert_eq!(
            &printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"),
            "6/60"
        );
        assert_eq!(
            &printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyzuuuuu"),
            "11/65"
        );
    }
}
