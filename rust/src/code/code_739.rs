fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![0; t.len()];
    let mut stack: Vec<usize> = vec![];

    for (i, j) in t.iter().enumerate() {
        while stack.len() != 0 && *j > t[stack[stack.len() - 1]] {
            ans[stack[stack.len() - 1]] = (i - stack[stack.len() - 1]) as i32;
            stack.pop();
        }
        stack.push(i);
    }

    ans
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_daily_temperatures() {
        let t = vec![73, 74, 75, 71, 69, 72, 76, 73];
        println!("{:?}", daily_temperatures(t));
    }
}
