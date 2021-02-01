// 题目描述
// 在一个n*m的只包含0和1的矩阵里找出一个不包含0的最大正方形，输出边长。

// 输入格式
// 输入文件第一行为两个整数n,m（1<=n,m<=100），接下来n行，每行m个数字，用空格隔开，0或1.

// 输出格式
// 一个整数，最大正方形的边长

// 输入输出样例

// 输入
// 4 4
// 0 1 1 1
// 1 1 1 0
// 0 1 1 0
// 1 1 0 1

// 输出
// 2

fn max_nozero_matrix(matrix: Vec<Vec<i32>>) -> i32 {
    // 前缀和
    let mut ret = vec![vec![0; matrix[0].len()]; matrix.len()];

    // 处理边界条件
    for (i, v) in matrix[0].iter().enumerate() {
        ret[0][i] = *v;
    }
    for (i, v) in matrix.iter().enumerate() {
        ret[i][0] = v[0]
    }

    let mut ans = 0_i32;

    // 计算 每个i j为右下角的最大正方形的边长
    let mut i = 1_usize;
    while i < matrix.len() {
        let mut j = 1_usize;
        while j < matrix[0].len() {
            use std::cmp::{max, min};
            if matrix[i][j] == 0 {
                ret[i][j] = 0
            } else {
                ret[i][j] = min(min(ret[i - 1][j], ret[i][j - 1]), ret[i - 1][j - 1]) + 1;
            }

            ans = max(ret[i][j], ans);

            j += 1;
        }
        i += 1;
    }

    ans
}

mod internal {
    pub fn pre_compute(matrix: &Vec<Vec<i32>>) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_nozero_matrix() {
        let matrix = [
            [0, 1, 1, 1].to_vec(),
            [1, 1, 1, 0].to_vec(),
            [0, 1, 1, 0].to_vec(),
            [1, 1, 0, 1].to_vec(),
        ]
        .to_vec();

        println!("{:?}", max_nozero_matrix(matrix));
    }
}
