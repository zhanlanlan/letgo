fn can_jump(nums: Vec<i32>) -> bool {
    let length = nums.len();
    let mut fartest = 0;

    for (i, j) in nums.iter().enumerate() {
        if i <= fartest {
            fartest = fartest.max(i + (*j) as usize);
            if fartest >= length - 1 {
                return true;
            }
        } else {
            return false;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_jump() {
        let nums = vec![3,2,1,0,4];

        println!("{:?}", can_jump(nums));
    }
}
