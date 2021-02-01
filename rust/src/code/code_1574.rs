// https://leetcode.com/problems/shortest-subarray-to-be-removed-to-make-array-sorted/

fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = arr.len() - 1;
    let mut ans;

    {
        for i in arr.iter().skip(1) {
            if arr[left] <= *i {
                left += 1;
            } else {
                break;
            }
        }
    }

    if left == arr.len() -1 {
        return 0;
    }

    {
        for j in arr.iter().rev().skip(1) {
            if arr[right] >= *j {
                right -= 1;
            } else {
                break;
            }
        }
    }

    ans = right.min(arr.len() - left - 1) as i32;

    let mut i = 0;
    let mut j = right;

    while i <= left && j <= arr.len() - 1 {
        if arr[i] <= arr[j] {
            ans = ans.min((j - i - 1) as i32);
            i += 1;
        } else {
            j += 1;
        }
    }

    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_length_of_shortest_subarray() {
        let arr = vec![1, 2, 3, 10, 4, 2, 3, 5];
        let arr2 = vec![1,2,3];


        println!("{:?}", find_length_of_shortest_subarray(arr2));
    }
}
