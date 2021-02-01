// https://vjudge.net/problem/AtCoder-joi2007ho_a#author=wuyudi

fn max_knum_sum(nums: Vec<i32>, k: usize) -> i32 {
    if k >= nums.len() {
        return nums.iter().sum();
    }

    let mut pre = vec![0; nums.len()];
    pre[0] = nums[0];
    for (i, v) in nums[..k].iter().enumerate().skip(1) {
        pre[i] = pre[i - 1] + v;
    }

    let mut ans = 0;
    {
        use std::cmp::max;
        let mut i = k;
        while i < nums.len() {
            pre[i] = pre[i - 1] - nums[i - k] + nums[i];
            ans = max(ans, pre[i]);
            i += 1;
        }
    }

    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_knum_sum() {
        let nums = vec![2, 5, -4, 10, 3];
        let k = 3;
        println!("{:?}", max_knum_sum(nums, k));
    }
}
