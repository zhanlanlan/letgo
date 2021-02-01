// https://leetcode.com/problems/cheapest-flights-within-k-stops/

fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    let ans = -1;

    // dp[i][j] 表示从 i城市 到 j城市 k次中转以下的最少花费。
    // let dp = [[0; 100]; 100];


    // 这应该是一道前缀和的题目
    // todo
    

    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_cheapest_price() {
        let n = 3;
        let edges = [[0,1,100].to_vec(),[1,2,100].to_vec(),[0,2,500].to_vec()].to_vec();
        let src = 0;
        let dst = 2;
        let k = 1;

        println!("{:?}", find_cheapest_price(n, edges, src, dst, k));
    }
}
