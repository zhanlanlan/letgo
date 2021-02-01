// https://leetcode.com/problems/rotate-image/

fn rotate_image(image: &mut Vec<Vec<i32>>) {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotate_image() {
        let mut image = [[1, 2, 3].to_vec(), [4, 5, 6].to_vec(), [7, 8, 9].to_vec()].to_vec();
        rotate_image(&mut image);
        println!("{:?}", image);
    }
}
