// 题目来源https://www.luogu.com.cn/problem/U53525

fn pre_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![0; nums.len()];
    ans[0] = nums[0];
    for (i, v) in nums.iter().enumerate().skip(1) {
        ans[i] = ans[i-1] + v
    }
    
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pre_sum() {
        let nums = vec![2, 6, 1, 9, 7, 3];

        println!("{:?}", pre_sum(nums));
    }
}
