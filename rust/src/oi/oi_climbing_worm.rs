/// 题目来自 [OI Wiki](https://oi-wiki.org/basic/simulate/)
fn climbing_worm(n: i32, u: i32, d: i32) -> i32 {
    let mut ans = 0;
    let mut climbed = 0;
    loop {
        climbed += u;
        ans += 1;
        if climbed >= n {
            return ans;
        }
        climbed -= d;
        ans += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_climbing_worm() {
        let n = 5433;
        let u = 9;
        let d = 4;

        let ans = climbing_worm(n, u, d);

        println!("{:?}", ans);
    }
}
