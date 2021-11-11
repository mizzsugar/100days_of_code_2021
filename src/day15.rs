// https://leetcode.com/problems/reverse-string
// https://leetcode.com/problems/reverse-string/discuss/354654/Rust-95%2B

pub fn reverse_string(s: &mut Vec<char>) {
    let len = s.len();
    if len > 0 {
        let mut midpoint = len / 2;
        if len % 2 != 0 {
            midpoint += 1;
        }
        for i in 0..midpoint {
            let j = len - i - 1;
            let buffer = s[i];
            s[i] = s[j];
            s[j] = buffer;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::reverse_string;
    #[test]
    fn test_reverse_string() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        let expected = vec!['o', 'l', 'l', 'e', 'h'];
        reverse_string(&mut s);
        assert_eq!(expected, s);
    }
}
