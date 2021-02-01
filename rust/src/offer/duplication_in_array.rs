use std::collections::hash_set::HashSet;

fn duplication_in_array1(mut nums: Vec<i32>, length: usize) -> i32 {
    nums.sort();

    let mut pre = nums[0];
    for i in nums[1..].iter() {
        if *i == pre {
            return pre;
        }

        pre = *i;
    }

    return 0;
}

fn duplication_in_array2(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort();

    let mut ans = vec![];

    let mut pre = nums[0];
    for i in nums[1..].iter() {
        if *i == pre {
            ans.push(pre);
        }
        pre = *i;
    }

    ans
}

fn duplication_in_array3(mut nums: Vec<i32>) -> Vec<i32> {
    let mut set = HashSet::<i32>::new();

    let mut ans = Vec::<i32>::new();

    for i in nums {
        if !set.insert(i) {
            ans.push(i);
        }
    }

    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_duplication_in_array1() {
        let nums = vec![2, 3, 1, 0, 2, 5, 3];
        let length = nums.len();

        let ans = duplication_in_array1(nums, length);

        println!("ans : {:?}", ans)
    }

    #[test]
    fn test_duplication_in_array2() {
        let nums = vec![2, 3, 1, 0, 2, 5, 3];

        let ans = duplication_in_array2(nums);

        println!("ans : {:?}", ans)
    }

    #[test]
    fn test_duplication_in_array3() {
        let nums = vec![2, 3, 1, 0, 2, 5, 3];

        let ans = duplication_in_array3(nums);

        println!("ans : {:?}", ans)
    }
}
