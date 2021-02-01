struct Solution {}
impl Solution {
    pub fn respace(dictionary: Vec<String>, sentence: String) -> i32 {
        if sentence.is_empty() {
            return 0;
        }
        let mut set = std::collections::HashSet::new();

        for s in dictionary.iter() {
            set.insert(s.as_str());
        }
        let s = sentence.as_str();
        let mut dp = vec![vec![0; sentence.len() + 1]; sentence.len() + 1];
        //第一步,扫描整个字符串,找出所有包含在字典中的dp[i][j],然后设置为0
        for i in 0..sentence.len() {
            for j in i..sentence.len() {
                if set.contains(&s[i..=j]) {
                    dp[i][j] = 0;
                } else {
                    dp[i][j] = j - i + 1;
                }
            }
        }
        // println!("dp={:?}", dp);
        //第二步,由短变长遍历整个dp,最终得出结果
        for step in 1..sentence.len() {
            for i in 0..sentence.len() - step {
                //从i开始往后走step步恰好是一个单词
                let total = &s[i..=(i + step)];
                if dp[i][i + step] == 0 {
                    continue;
                }
                // if total == "jesslooked" {
                //     println!("total={}", total);
                // }
                //没有组成一个单词,就开始切分,找出切分组合中最小的情况
                let mut expected = dp[i][i + step];
                for j in 0..step {
                    if dp[i][i + j] + dp[i + j + 1][i + step] < expected {
                        expected = dp[i][i + j] + dp[i + j + 1][i + step]
                    }
                }
                dp[i][i + step] = expected;
                // println!("total={},dp[{}][{}]={}", total, i, step, dp[i][step]);
            }
        }
        dp[0][sentence.len() - 1] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {}
}
