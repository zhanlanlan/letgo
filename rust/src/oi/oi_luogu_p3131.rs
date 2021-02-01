// https://www.luogu.com.cn/problem/P3131

fn subsequences_summing_to_sevens(nums: Vec<i32>) -> usize {
    // 处理边界条件
    match nums.len() {
        0 => return 0,
        1 => {
            if nums[0] % 7 == 0 {
                return 1;
            } else {
                return 0;
            }
        }
        _ => (),
    }

    let mut pre = vec![0; nums.len()];
    let mut ans = 0;

    // 首先处理前缀和
    pre[0] = nums[0];
    for (i, v) in nums.iter().enumerate().skip(1) {
        pre[i] = pre[i - 1] + nums[i];
    }

    // 假设区间的长度就是数组的长度
    if pre[nums.len() - 1] % 7 == 0 {
        return nums.len();
    }

    // 然后缩短区间假设
    {
        let mut i = nums.len() - 1;
        while i > 0 {
            let mut j = i;
            while j < nums.len() {
                if (pre[j] - pre[j - i]) % 7 == 0 {
                    return i;
                }
                j += 1;
            }
            i -= 1;
        }
    }

    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_subsequences_summing_to_sevens() {
        let nums = vec![3, 5, 1, 6, 2, 14, 10];

        println!("{:?}", subsequences_summing_to_sevens(nums));
    }
}
