// https://leetcode.com/problems/triangle/

fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let mut ans;
    let mut dp = vec![vec![0; triangle.len()]; triangle.len()];

    dp[0][0] = triangle[0][0];
    let mut i = 1;
    while i < triangle.len() {
        dp[i][0] = dp[i - 1][0] + triangle[i][0];
        let mut j = 1;
        while j < i {
            dp[i][j] = std::cmp::min(dp[i - 1][j], dp[i - 1][j - 1]) + triangle[i][j];
            j += 1;
        }
        dp[i][i] = dp[i - 1][i - 1] + triangle[i][i];
        i += 1;
    }

    ans = dp[triangle.len() - 1][0];
    for x in dp[triangle.len() - 1].iter() {
        ans = ans.min(*x);
    }

    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_minimum_total() {
        #[rustfmt::skip]
        let triangle = vec![
            vec![2],
            vec![3,4],
            vec![6,5,7],
            vec![4,1,8,3]
       ];

        println!("{:#?}", minimum_total(triangle));
    }
}
