// https://leetcode.com/problems/new-21-game/

fn new21_game(n: i32, k: i32, w: i32) -> f64 {
    if k == 0 {
        return 1.0;
    }

    if (n - k + 1) >= w {
        return 1.0;
    }

    let mut dp = vec![0_f64; (k + w) as usize];

    {
        let mut i = k;
        while i <= n && i <= k + w - 1 {
            dp[i as usize] = 1.0;
            i += 1;
        }
        dp[(k - 1) as usize] = (w.min(n - k + 1) / w) as f64;
    }

    {
        let mut sum_prob = (n - k + 1) as f64;
        let mut i = k - 1;
        while i >= 0 {
            dp[i as usize] = sum_prob / w as f64;
            sum_prob = sum_prob - dp[(i + w) as usize] + dp[i as usize];
            i -= 1;
        }
    }

    dp[0]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new21_game() {
        let (n, k, w) = (21, 17, 10);

        println!("{:?}", new21_game(n, k, w));
    }
}
