fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut pre_sum = vec![vec![0; mat[0].len()]; mat.len()];
    let mut res = vec![vec![0; mat[0].len()]; mat.len()];

    for (i, j) in mat.iter().enumerate() {
        for (m, n) in j.iter().enumerate() {
            pre_sum[i][m] = get(&pre_sum, i as i32 - 1, m as i32)
                + get(&pre_sum, i as i32, m as i32 - 1)
                - get(&pre_sum, i as i32 - 1, m as i32 - 1)
                + mat[i][m];
        }

        // todo
    }

    println!("{:?}", pre_sum);

    todo!()
}

fn get(mat: &Vec<Vec<i32>>, x: i32, y: i32) -> i32 {
    if x < 0 {
        return 0;
    }

    if y < 0 {
        return 0;
    }

    mat[x as usize][y as usize]
}

fn get2(mat: &Vec<Vec<i32>>, x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matrix_block_sum() {
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let k = 1;

        println!("{:?}", matrix_block_sum(mat, k));
    }
}
