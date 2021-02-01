use std::char::ToLowercase;

// https://leetcode.com/problems/transpose-matrix/

fn transpose(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut b = vec![vec![0; a.len()]; a[0].len()];

    let mut i = 0usize;
    while i < a.len() {
        let mut j = 0usize;
        while j < a[0].len() {
            b[j][i] = a[i][j];

            j += 1;
        }

        i += 1;
    }

    b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_transpose() {
        let a: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        println!("{:?}", transpose(a));
    }
}
