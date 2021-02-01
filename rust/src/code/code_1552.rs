// https://leetcode.com/problems/magnetic-force-between-two-balls/

fn max_distance(position: Vec<i32>, m: i32) -> i32 {
    let mut position = position;
    position.sort_unstable();

    let (mut left, mut right) = (0, position.last().unwrap().clone());

    let mut ans = 0;
    let mut mid;

    while left <= right {
        mid = (left + right) / 2;

        let mut i = 1_usize;
        let mut mag = 0;
        let mut count = 1; // 0 号位置上放了一个 所以从1开始
        while i < position.len() {
            mag += (position[i] - position[i - 1]).abs();
            i += 1;
            if mag >= mid {
                mag = 0;
                count += 1;
            }

            if count >= m {
                break;
            }

        }

        if count >= m {
            ans = mid;
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_distance() {
        let position = vec![1, 2, 3, 4, 7];
        let m = 3;

        println!("{:#?}", max_distance(position, m));
    }
}
