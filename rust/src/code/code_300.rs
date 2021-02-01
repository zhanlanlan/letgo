// https://leetcode.com/problems/longest-increasing-subsequence/

fn length_of_lis(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    use std::cmp::max;

    let mut ans = 1;
    // dp[i] 表示以nums[i]结尾的最长子串的长度。
    let mut dp = vec![0; nums.len()];
    dp[0] = 1;

    let mut i = 1_usize;
    while i < nums.len() {
        dp[i] = 1;

        let mut j = 0_usize;
        while j < i {
            if nums[i] > nums[j] {
                dp[i] = max(dp[i], dp[j] + 1);
            }

            j += 1;
        }

        ans = max(ans, dp[i]);

        i += 1;
    }

    ans
}

fn length_of_lis2(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    let mut ans = 1;
    let len = nums.len();

    let mut d = vec![0; 2];
    d[1] = nums[0];

    for i in nums {
        if i > d[ans] {
            ans += 1;
            d.push(i)
        } else {
            let (mut l, mut r, mut pos) = (1_usize, ans, 0_usize);
            while l <= r {
                let mid = (l + r) >> 1;
                if d[mid] < i {
                    pos = mid;
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }

            d[pos + 1] = i;
        }
    }

    ans as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_length_of_lis() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];

        println!("{:?}", length_of_lis(nums));
    }

    #[test]
    fn test_length_of_lis2() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];

        println!("{:?}", length_of_lis2(nums));
    }
}
