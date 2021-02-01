fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
    nums1.sort();
    nums2.sort();
    let (mut p1, mut p2) = (0usize, 0usize);
    let mut ans = Vec::new();
    loop {
        if nums1[p1] == nums2[p2] {
            ans.push(nums2[p2])
        }
        if nums1[p1] > nums2[p2] {
            p2 += 1;
        } else {
            p1 += 1;
        }

        if p1 >= nums1.len() || p2 >= nums2.len() {
            break;
        }
    }

    ans
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_intersect() {
        let nums1 = vec![1, 2, 3, 4, 215, 3, 4, 4, 32, 4, 1215, 5, 31];
        let nums2 = vec![1215, 2, 15, 4, 3, 64, 31, 215, 25, 43, 6, 32, 4, 2, 1];

        println!("{:?}", intersect(nums1, nums2));
    }
}
