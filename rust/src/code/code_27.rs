fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let mut idx = 0usize;

    while idx < nums.len() {
        if nums[idx] == val {
            nums.remove(idx);
        } else {
            idx += 1
        }
    }

    nums.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_element() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let mut val = 2;
        let ans = remove_element(&mut nums, val);
        println!("result : {:?}, ans : {:?}", nums, ans);
    }
}
