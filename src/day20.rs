// https://leetcode.com/problems/string-to-integer-atoi
/// https://leetcode.com/problems/string-to-integer-atoi/discuss/294603/rust-simple-solution
use std::i32;

pub fn my_atoi(str: String) -> i32 {
    let n = str
        .chars()
        .skip_while(|x| x.is_ascii_whitespace())
        .enumerate()
        .take_while(|(i, c)| match c {
            '0'..='9' => true,
            '+' | '-' => *i == 0 as usize,
            _ => false,
        })
        .map(|(_, c)| c)
        .collect::<String>();

    if n.is_empty() {
        return 0;
    }

    match n.parse::<i32>() {
        Ok(n) => n,
        Err(ref e) => {
            let signs = n.chars().take_while(|&c| c == '+' || c == '-');
            let n_signs = signs.count();

            if 1 < n_signs {
                0
            } else if n_signs == 1 && 1 == n.len() {
                std::i32::MIN
            } else {
                std::i32::MAX
            }
        }
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::my_atoi;
    #[test]
    fn test_my_atoi() {
        let s = String::from("4193 with words");
        assert_eq!(4193, my_atoi(s));
    }
}
