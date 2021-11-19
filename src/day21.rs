// https://leetcode.com/problems/implement-strstr/
// https://leetcode.com/problems/implement-strstr/discuss/500539/Rust-0ms

pub fn str_str(haystack: String, needle: String) -> i32 {
    match haystack.find(&needle) {
        Some(n) => n as i32,
        None => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::str_str;
    #[test]
    fn test_str_str() {
        let haystack = String::from("hello");
        let needle = String::from("ll");
        let expected: i32 = 2;
        assert_eq!(expected, str_str(haystack, needle));
    }

    #[test]
    fn test_str_str_no_match() {
        let haystack = String::from("aaaaa");
        let needle = String::from("bba");
        let expected: i32 = -1;
        assert_eq!(expected, str_str(haystack, needle));
    }

    #[test]
    fn test_str_str_empty() {
        let haystack = String::from("");
        let needle = String::from("");
        let expected: i32 = 0;
        assert_eq!(expected, str_str(haystack, needle));
    }
}
