// https://leetcode.com/problems/valid-palindrome

pub fn is_palindrome(s: String) -> bool {
    let s_chars: Vec<char> = s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let s_1: Vec<&char> = s_chars.iter().collect();
    let s_2: Vec<&char> = s_chars.iter().rev().collect();
    s_1 == s_2
}

#[cfg(test)]
mod tests {
    use super::is_palindrome;
    #[test]
    fn test_is_palindrome() {
        let s = String::from("A man, a plan, a canal: Panama");
        assert!(is_palindrome(s));
    }
}
