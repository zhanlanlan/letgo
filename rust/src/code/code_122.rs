use std::cmp::max;

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut ans = 0i32;
    let mut i = 1;
    while i < prices.len() {
        if prices[i] > prices[i - 1] {
            ans += prices[i] - prices[i - 1];
        }
        i += 1;
    }

    return ans;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_max_profit() {
        println!("{:?}", max_profit(vec![7, 1, 5, 3, 6, 4]));
    }
}
