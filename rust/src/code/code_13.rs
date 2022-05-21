pub fn roman_to_int(s: String) -> i32 {
    const DIGITS: [(&[u8], i32); 13] = [
        (b"M", 1000),
        (b"CM", 900),
        (b"D", 500),
        (b"CD", 400),
        (b"C", 100),
        (b"XC", 90),
        (b"L", 50),
        (b"XL", 40),
        (b"X", 10),
        (b"IX", 9),
        (b"V", 5),
        (b"IV", 4),
        (b"I", 1),
    ];

    let mut result = 0;
    let s = s.into_bytes();
    let mut slice = s.as_slice();
    let (mut digit, mut digits) = DIGITS.split_first().unwrap();

    loop {
        if slice.starts_with(digit.0) {
            result += digit.1;
            slice = &slice[digit.0.len()..];

            if slice.is_empty() {
                break;
            }
        } else {
            let (new_digit, new_digits) = digits.split_first().unwrap();

            digit = new_digit;
            digits = new_digits;
        }
    }

    result
}

// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000

pub fn roman_to_int2(s: String) -> i32 {
    let mut ans = 0;
    let mut pre_value = 99999;
    let mut table = [0_i32; 250];
    table['I' as usize] = 1;
    table['V' as usize] = 5;
    table['X' as usize] = 10;
    table['L' as usize] = 50;
    table['C' as usize] = 100;
    table['D' as usize] = 500;
    table['M' as usize] = 1000;

    let s = s.into_bytes();
    for (i, v) in s.iter().enumerate() {
        let val = table[*v as usize];
        if val != 0 {
            if val > pre_value {
                ans = ans - pre_value - pre_value + val;
            } else {
                ans += val;
            }

            pre_value = val;
        }
    }

    return ans;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_roman_to_int() {}

    #[test]
    fn test_roman_to_int2() {
        assert_eq!(roman_to_int2("III".into()), 3);
        assert_eq!(roman_to_int2("IV".into()), 4);
        assert_eq!(roman_to_int2("IX".into()), 9);
        assert_eq!(roman_to_int2("LVIII".into()), 58);
    }
}
