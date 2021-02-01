// https://leetcode.com/problems/maximum-swap/

fn maximum_swap(num: i32) -> i32 {
    // 边界条件
    if num <= 10 {
        return num;
    }

    // 转换为数字数组
    let mut num = num;
    let mut nums = vec![];
    while num > 0 {
        nums.push(num % 10);
        num = num / 10;
    }

    

    println!("{:?}", nums);

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_maximum_swap() {
        let num = 2736;

        println!("{:?}", maximum_swap(num));
    }
}
