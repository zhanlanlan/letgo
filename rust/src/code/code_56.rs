fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.len() <= 1 {
        return intervals;
    }

    let mut ans = vec![];

    let mut intervals = intervals;
    intervals.sort_unstable_by_key(|x| {
        return (x[0], x[1]);
    });

    ans.push(intervals[0].clone());

    for i in intervals.into_iter().skip(1) {
        if i[0] <= ans.last().unwrap()[1]  {
            ans.last_mut().unwrap()[1] = i[1].max(ans.last_mut().unwrap()[1])
        } else {
            ans.push(i)
        }
    }

    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];

        println!("{:?}", merge(intervals));
    }
}
