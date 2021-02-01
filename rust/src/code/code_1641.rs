fn count_vowel_strings(n: i32) -> i32 {
    let mut dp = vec![vec![0; 5]; n as usize + 1];

    if n == 0 {
        return 0;
    }

    dp[1][0] = 1;
    dp[1][1] = 1;
    dp[1][2] = 1;
    dp[1][3] = 1;
    dp[1][4] = 1;

    let mut i = 2;
    while i <= n as usize {
        dp[i][0] = dp[i - 1][0];
        dp[i][1] = dp[i - 1][0] + dp[i - 1][1];
        dp[i][2] = dp[i - 1][0] + dp[i - 1][1] + dp[i - 1][2];
        dp[i][3] = dp[i - 1][0] + dp[i - 1][1] + dp[i - 1][2] + dp[i - 1][3];
        dp[i][4] = dp[i - 1][0] + dp[i - 1][1] + dp[i - 1][2] + dp[i - 1][3] + dp[i - 1][4];

        i += 1;
    }

    return dp[n as usize][0]
        + dp[n as usize][1]
        + dp[n as usize][2]
        + dp[n as usize][3]
        + dp[n as usize][4];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_vowel_strings() {
        let n = 2;

        println!("{:?}", count_vowel_strings(n));
    }
}
