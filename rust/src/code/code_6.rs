struct Solution();

impl Solution {
    fn convert(s: String, num_rows: i32) -> String {
        if num_rows < 2 {
            return s;
        }

        let mut anss = vec![String::new(); num_rows as usize];
        let (mut i, mut flag) = (0, -1);
        for c in s.chars() {
            anss[i as usize].push(c);
            if i == 0 || i == num_rows - 1 {
                flag = -flag
            }
            i += flag
        }

        let mut ans = String::new();

        for s in anss {
            ans.push_str(&s)
        }

        return ans;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_solution() {
        let s = String::from("AB");
        let num_rows = 1;
        let ans = Solution::convert(s, num_rows);
        println!("{:?}", ans)
    }
}
