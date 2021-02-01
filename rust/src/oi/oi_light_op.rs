/// 题目源自 [OI Wiki](http://bailian.openjudge.cn/practice/2811/)
fn light_op(ligts: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut i = 0;
    while i < 2i32.pow(6) {
        let mut first_row_op = [0, 0, 0, 0, 0, 0]; // 第一行的操作方式
        for ii in 0..6 {
            let value = i >> ii & 1;
            first_row_op[ii] = value;
        }

        let mut result = ligts.clone(); // 本轮递推的结果
        let mut result_op = vec![vec![0; 6]; 5]; // 本轮所有的操作

        // 首先进行第一轮的操作
        for (idx, val) in first_row_op.iter().enumerate() {
            if *val == 0 {
                op(&mut result, 0, idx);
                result_op[0][idx] = 1; // 记录开关按下
            }
        }

        // 然后进行接下来的操作
        for row in 1..5 {
            let mut column = 0usize;
            while column < 6 {
                if result[row - 1][column] == 1 {
                    op(&mut result, row, column);
                    result_op[row][column] = 1; // 记录开关按下
                }
                column += 1;
            }
        }

        // 判断操作过后结果是否通过
        if let [0, 0, 0, 0, 0, 0] = result[4].as_slice() {
            return result_op;
        }

        i += 1;
    }
    unreachable!();
}

fn op(lights: &mut Vec<Vec<i32>>, row: usize, column: usize) {
    lights[row][column] = swap0and1(lights[row][column]);
    if row > 0 {
        lights[row - 1][column] = swap0and1(lights[row - 1][column]) // 上方
    }
    if row < 4 {
        lights[row + 1][column] = swap0and1(lights[row + 1][column]) // 下方
    }
    if column > 0 {
        lights[row][column - 1] = swap0and1(lights[row][column - 1]) //左边
    }
    if column < 5 {
        lights[row][column + 1] = swap0and1(lights[row][column + 1]) //右边
    }
}

fn swap0and1(num: i32) -> i32 {
    return (num + 1) % 2; // 0 + 1 % 2 = 1; 1 + 1 % 2 = 0;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_light_op() {
        let nums = vec![
            vec![0, 1, 1, 0, 1, 0],
            vec![1, 0, 0, 1, 1, 1],
            vec![0, 0, 1, 0, 0, 1],
            vec![1, 0, 0, 1, 0, 1],
            vec![0, 1, 1, 1, 0, 0],
        ];
        let ans = light_op(nums);
        println!("{:?}", ans);
    }

    #[test]
    fn test_get_bit() {
        let mut i = 0;
        while i < 2i32.pow(6) {
            let mut arr = [0, 0, 0, 0, 0, 0];
            for ii in 0..6 {
                let value = i >> ii & 1;
                arr[ii] = value;
            }
            println!("{:?}", arr);
            i += 1;
        }
    }
}
