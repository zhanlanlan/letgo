use std::collections::VecDeque;

fn letter_combinations(digits: String) -> Vec<String> {
    if digits.len() == 0 {
        return vec![];
    }

    let mut ans;
    let mut x = VecDeque::<String>::new();
    x.push_back(String::from(""));

    let mut dic = [""; 60];
    dic['2' as usize] = "abc";
    dic['3' as usize] = "def";
    dic['4' as usize] = "ghi";
    dic['5' as usize] = "jkl";
    dic['6' as usize] = "mno";
    dic['7' as usize] = "pqrs";
    dic['8' as usize] = "tuv";
    dic['9' as usize] = "wxyz";

    for c in digits.as_bytes() {
        let letter = dic[*c as usize];
        let x_size = x.len();
        for i in 0..x_size {
            let tmp = x.pop_front().unwrap();
            for m in letter.chars() {
                x.push_back({
                    let mut t = tmp.clone();
                    t.push(m);
                    t
                });
            }
        }
    }

    ans = x.into();
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_letter_combinations() {
        let digits = String::from("23");

        println!("{:?}", letter_combinations(digits));
    }
}
