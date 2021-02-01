fn rotate1(nums: &mut Vec<i32>, k: i32) {
    let position = nums.len() - (k as usize % nums.len());
    let mut ans = vec![];

    for i in nums[position..nums.len()].into_iter() {
        ans.push(*i)
    }

    for i in nums[0..position].iter() {
        ans.push(*i)
    }

    *nums = ans
}

fn rotate2(nums: &mut Vec<i32>, k: i32) {
    if k == 0 {
        return;
    }

    let length = nums.len();

    if length == 1 {
        return;
    }

    let offset = length - (k as usize % length);
    let mut move_count = 0usize;

    let mut start = 0usize;
    while move_count < length {
        let tmp = nums[start];
        let mut from = offset + start;
        let mut to = start;
        while from % length != start {
            // println!("from : {}, to : {}", from, to);
            move_count += 1;
            nums[to] = nums[from];
            to = from;
            from = (from + offset) % length;
        }
        // println!("swap tmp : {}", start);
        nums[(k as usize % length) + start] = tmp;
        move_count += 1;

        start += 1;
    }

    // let offset = nums.len() - (k as usize % nums.len());

    // let mut move_count = 0;

    // let mut start = 0usize;
    // while move_count < nums.len() {
    //     let tmp = nums[start];
    //     let mut from = offset + start;
    //     let mut to = start;
    //     while from % nums.len() != start {
    //         println!("from : {}, to : {}", from, to);
    //         move_count += 1;
    //         nums[to] = nums[from];
    //         from = (from + offset) % nums.len();
    //         to = (to + offset) % nums.len();
    //     }
    //     nums[k as usize + start] = tmp;
    //     start += 1;
    // }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_rotate1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let k = 3;
        rotate1(&mut nums, k);
        println!("{:?}", nums);
    }

    #[test]
    fn test_rotate2() {
        let mut nums = vec![-1, -100, 3, 99];
        let k = 2;
        rotate2(&mut nums, k);
        println!("{:?}", nums);
    }

    #[test]
    fn test_something() {
        println!("{}", 3usize / 2usize);
    }
}
