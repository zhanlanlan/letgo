use std::collections::HashMap;

fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    let mut freq = HashMap::new();
    for i in tasks.iter() {
        let e = freq.entry(*i).or_insert(0);
        *e += 1;
    }

    let m = freq.len();
    let (mut next_valid, mut rest) = (vec![], vec![]);
    for i in freq.values() {
        next_valid.push(1);
        rest.push(*i);
    }

    let mut ans = 0;
    for i in 0..tasks.len() {
        ans += 1;

        let mut min_next_valid = i32::MAX;
        for j in 0..m {
            if rest[j] != 0 {
                min_next_valid = min_next_valid.min(next_valid[j]);
            }
        }

        ans = ans.max(min_next_valid);
        let mut best = -1;
        for j in 0..m {
            if rest[j] != 0 && next_valid[j] <= ans {
                if best == -1 || rest[j] > rest[best as usize] {
                    best = j as i32;
                }
            }
        }

        next_valid[best as usize] = ans + n + 1;
        rest[best as usize] -= 1;
    }

    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_least_interval() {
        let tasks = ['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'];
        let n = 2;

        println!("{:?}", least_interval(tasks.into(), n));
    }
}
