fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() < 3 {
        return vec![];
    }

    let mut ans = vec![];
    let mut nums = nums;
    nums.sort();

    'a: for (i, j) in nums.iter().enumerate() {
        if *j > 0 {
            return ans;
        }

        if i != 0 && nums[i] == nums[i - 1] {
            continue 'a;
        }

        let mut second = i + 1;
        let mut third = nums.len() - 1;
        let target = -j;
        'b: while second < nums.len() {
            if second > i + 1 && nums[second] == nums[second - 1] {
                second += 1;
                continue 'b;
            }

            while third > second && nums[third] + nums[second] > target {
                third -= 1;
            }

            if second == third {
                break;
            }

            if nums[second] + nums[third] == target {
                ans.push(vec![nums[i], nums[second], nums[third]]);
            }

            second += 1;
        }
    }

    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_three_sum() {
        let nums = vec![-1, 0, 1, 2, -1, -4];

        println!("{:?}", three_sum(nums));
    }
}
