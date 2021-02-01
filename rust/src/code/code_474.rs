// https://leetcode.com/problems/ones-and-zeroes/

fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    use internal::*;
    use std::cmp::{max, min};

    // dp[i][j] 表示 最多包含 i个0 和 j个1 的最大字符串数量
    // 因此 dp[m][n] 就是最终答案 
    let mut dp = vec![vec![0; n as usize + 1]; m as usize + 1];
    let ans = 0;

    for s in strs.iter() {
        let mut i = m;
        let (zero, one) = count_zero_one(s);
        while i >= zero {
            let mut j = n;
            while j >= one {
                dp[i as usize][j as usize] = max(
                    dp[i as usize][j as usize],
                    1 + dp[(i - zero) as usize][(j - one) as usize],
                );
                j -= 1;
            }

            i -= 1;
        }
    }

    dp[m as usize][n as usize]
}

mod internal {

    #[inline]
    pub fn count_zero_one(s: &str) -> (i32, i32) {
        let (mut one, mut zero) = (0_i32, 0_i32);
        for c in s.chars() {
            if c == '1' {
                one += 1
            } else {
                zero += 1
            }
        }
        (zero, one)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_max_form() {
        let strs = vec![
            "10".to_string(),
            "0001".to_string(),
            "111001".to_string(),
            "1".to_string(),
            "0".to_string(),
        ];
        let (m, n) = (5, 3);

        println!("{:?}", find_max_form(strs, m, n))
    }
}
