// https://leetcode.com/problems/roman-to-integer/
// https://leetcode.com/discuss/explore/february-leetcoding-challenge-2021/1074181/roman-to-integer-rust-matching-0ms

fn roman_to_int(s: String) -> i32 {
    let mut c = s.chars().peekable();
    let mut total = 0;
    loop {
        total += match c.next() {
            Some('I') => match c.peek() {
                Some('V') => {
                    c.next();
                    4
                }
                Some('X') => {
                    c.next();
                    9
                }
                _ => 1,
            },
            Some('V') => 5,
            Some('X') => match c.peek() {
                Some('L') => {
                    c.next();
                    40
                }
                Some('C') => {
                    c.next();
                    90
                }
                _ => 10,
            },
            Some('L') => 50,
            Some('C') => match c.peek() {
                Some('D') => {
                    c.next();
                    400
                }
                Some('M') => {
                    c.next();
                    900
                }
                _ => 100,
            },
            Some('D') => 500,
            Some('M') => 1000,
            None => break,
            _ => 0,
        };
    }
    total
}

#[cfg(test)]
mod tests {
    use super::roman_to_int;

    #[test]
    fn test_roman_to_int() {
        let expected = 58;
        let actual = roman_to_int(String::from("LVIII"));
        assert_eq!(expected, actual)
    }
}
