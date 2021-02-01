fn enumerate(nums: Vec<i32>, k: i32) {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_enumerate() {
        let nums = vec![1, 2, 3, 3, 32, 1, 42, 15, 2, 15, 2];
        let k = 7;
        enumerate(nums, k);
    }
}
