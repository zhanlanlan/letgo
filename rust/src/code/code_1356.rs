// https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits/

use std::cmp::Ordering;

fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
    let mut ans = arr;

    let mut f = |l: &i32, r: &i32| -> Ordering {
        let l_count = count_one(*l);
        let r_count = count_one(*r);

        if l_count > r_count {
            return Ordering::Greater;
        } else if l_count < r_count {
            return Ordering::Less;
        }

        if l > r {
            return Ordering::Greater;
        }

        return Ordering::Less;
    };

    ans.sort_unstable_by(f);

    ans
}

fn count_one(mut num: i32) -> i32 {
    let mut count = 0;
    while num > 0 {
        count += num & 1;
        num = num >> 1;
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort_by_bits() {
        let arr = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];

        println!("{:?}", sort_by_bits(arr));
    }
}
