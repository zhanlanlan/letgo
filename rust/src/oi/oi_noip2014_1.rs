/// 题目来自 [OI Wiki](https://uoj.ac/problem/15)
fn solution(a_input: Vec<i32>, b_input: Vec<i32>, mut total_round: i32) -> (i32, i32) {
    let mut table = [[0; 5]; 5];
    table[0][0] = 0;
    table[0][1] = 2;
    table[0][2] = 1;
    table[0][3] = 1;
    table[0][4] = 2;

    table[1][0] = 1;
    table[1][1] = 0;
    table[1][2] = 2;
    table[1][3] = 1;
    table[1][4] = 2;

    table[2][0] = 2;
    table[2][1] = 1;
    table[2][2] = 0;
    table[2][3] = 2;
    table[2][4] = 1;

    table[3][0] = 2;
    table[3][1] = 2;
    table[3][2] = 1;
    table[3][3] = 0;
    table[3][4] = 1;

    table[4][0] = 1;
    table[4][1] = 1;
    table[4][2] = 2;
    table[4][3] = 2;
    table[4][4] = 0;

    let mut round = 0;

    let mut a_win = 0;
    let mut b_win = 0;

    while round < total_round {
        let a_out = a_input[round as usize % a_input.len()];
        let b_out = b_input[round as usize % b_input.len()];

        let result = table[a_out as usize][b_out as usize];

        if result == 1 {
            a_win += 1;
        } else if result == 2 {
            b_win += 1;
        }

        round += 1;
    }

    return (a_win, b_win);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        let a_input = vec![0, 1, 2, 3, 4];
        let b_input = vec![1, 0, 3, 2, 4];
        let total_round = 9;

        let ans = solution(a_input, b_input, total_round);

        println!("{:?}", ans);
    }
}
