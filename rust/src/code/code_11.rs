fn max_area(height: Vec<i32>) -> i32 {
    let (mut l, mut r, mut ans) = (0usize, height.len() - 1, 0);

    while l < r {
        let area = std::cmp::min(height[l], height[r]) as usize * (r - l);

        ans = std::cmp::max(ans, area);

        if height[l] <= height[r] {
            l += 1;
        } else {
            r -= 1;
        }
    }

    return ans as i32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_area() {}
}
