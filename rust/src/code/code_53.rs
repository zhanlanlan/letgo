fn max_sub_array(nums: Vec<i32>) -> i32 {
    if nums.len() < 1 {
        return 0;
    }

    use std::cmp::max;
    let mut dp = vec![0i32; nums.len()];

    dp[0] = nums[0];
    let mut ans = nums[0];

    let mut i = 1usize;
    while i < nums.len() {
        dp[i] = max(nums[i], dp[i - 1] + nums[i]);
        ans = max(ans, dp[i]);
        i += 1;
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_max_sub_array() {
        println!("{:?}", max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]));
    }
}
