fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    let mut ans;
    let mut left = 0;
    while left < nums.len() - 1 && nums[left] <= nums[left + 1] {
        left += 1;
    }

    let mut right = nums.len() - 1;
    while right > 0 && nums[right] >= nums[right - 1] {
        right -= 1;
    }

    if left >= right {
        return 0;
    }

    let (mut max_num, mut min_num) = (std::i32::MIN, std::i32::MAX);
    for i in nums[left..right + 1].iter() {
        max_num = max_num.max(*i);
        min_num = min_num.min(*i);
    }

    while left > 0 && nums[left - 1] > min_num {
        left -= 1
    }

    while right < nums.len() - 1 && nums[right + 1] < max_num {
        right += 1
    }

    ans = if right - left > 0 {
        right - left + 1
    } else {
        0
    };

    println!("left: {:?}, right: {:?}", left, right);

    ans as i32
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_unsorted_subarray() {
        let nums = [1, 2, 3, 3, 3];

        println!("{:?}", find_unsorted_subarray(nums.into()));
    }
}
