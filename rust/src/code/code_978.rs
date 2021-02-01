// https://leetcode.com/problems/longest-turbulent-subarray/

fn max_turbulence_size(a: Vec<i32>) -> i32 {
    use std::cmp::max;
    // 边界条件
    match a.len() {
        0 => return 0,
        1 => return 1,
        _ => (),
    }

    let mut ans = 0;
    // 令 dp[i]
    let mut dp = vec![0; a.len()];
    dp[0] = 1;
    // flag = -1 表示 <  ; flag = 0 表示 = ; flag = 1 表示 > ;
    let mut flag = 0;


    let mut i = 1_usize;
    while i < a.len() {
        let f2 = get_flag(a[i - 1], a[i]);
        if f2 != flag && f2 != 0 {
            dp[i] = dp[i - 1] + 1;
        } else if f2 == 0 {
            dp[i] = 1;
        } else {
            dp[i] = 2;
        }

        ans = max(ans, dp[i]);

        flag = f2;
        i += 1;
    }

    ans
}

fn get_flag(num1: i32, num2: i32) -> i32 {
    if num1 < num2 {
        return -1;
    }

    if num1 == num2 {
        return 0;
    }

    return 1;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_turbulence_size() {
        let a = vec![9,4,2,10,7,8,8,1,9];

        println!("{:?}", max_turbulence_size(a));
    }
}
