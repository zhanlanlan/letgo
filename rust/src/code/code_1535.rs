fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
    use std::cmp::max;

    let mut ans = max(arr[0], arr[1]);
    
    let mut max_num = ans;
    let mut winned = 1;

    if k == 1 {
        return ans;
    }

    let mut i = 2_usize;
    while i < arr.len() {
        if ans > arr[i] {
            winned += 1;
            if winned == k {
                return ans;
            }
        } else {
            ans = arr[i];
            winned = 1;
        }

        max_num = max(max_num, arr[i]);
        i += 1;
    }

    max_num
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_winner() {
        let arr = vec![2, 1, 3, 5, 4, 6, 7];
        let k = 2;

        println!("{:?}", get_winner(arr, k));
    }
}
