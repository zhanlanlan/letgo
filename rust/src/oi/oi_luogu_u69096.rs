// https://www.luogu.com.cn/problem/U69096

fn rev_pre_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![0; nums.len()];
    ans[0] = nums[0];

    for (i, v) in nums.iter().enumerate().skip(1) {
        ans[i] = nums[i] - nums[i-1];
    }

    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rev_pre_sum() {
        let nums = vec![2, 8, 9, 18, 25, 28];

        println!("{:?}", rev_pre_sum(nums));
    }
}
