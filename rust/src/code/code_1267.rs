// https://leetcode.com/problems/count-servers-that-communicate/

fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    let mut count_row = vec![0; grid.len()];
    let mut count_column = vec![0; grid[0].len()];

    for (i, j) in grid.iter().enumerate() {
        for (m, n) in j.iter().enumerate() {
            if *n == 1 {
                count_row[i] += 1;
                count_column[m] += 1;
            }
        }
    }

    for (i, j) in grid.iter().enumerate() {
        for (m, n) in j.iter().enumerate() {
            if *n == 1 && (count_row[i] > 1 || count_column[m] > 1) {
                ans += 1;
            }
        }
    }

    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_servers() {
        let grid = vec![
            vec![1, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1],
        ];

        println!("{:#?}", count_servers(grid));
    }
}
