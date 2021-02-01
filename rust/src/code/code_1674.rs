// https://leetcode.com/problems/minimum-moves-to-make-array-complementary/
use std::cmp::{max, min};

fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
    let n = nums.len();
    let mut delta = vec![0; (limit * 2 + 2) as usize];

    let mut i = 0_usize;
    while i < n / 2 {
        let a = min(nums[i], nums[n - i - 1]);
        let b = max(nums[i], nums[n - i - 1]);
        delta[2] += 2;
        delta[a as usize + 1] -= 1;
        delta[(a + b) as usize] -= 1;
        delta[(a + b + 1) as usize] += 1;
        delta[(b + limit + 1) as usize] += 1;

        i += 1;
    }

    let mut ans = n;
    let mut cur = 0;
    for i in delta.iter().skip(2) {
        cur += *i;
        ans = min(cur, ans)
    }


    ans as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_moves() {
        let nums = vec![1, 2, 2, 1];
        let limit = 2;

        println!("{:#?}", min_moves(nums, limit));
    }
}
