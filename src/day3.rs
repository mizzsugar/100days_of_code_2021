// https://leetcode.com/problems/palindrome-number
pub fn is_palindrome(x: i32) -> bool {
    if x <= 0 || x % 10 == 0 {
        return false;
    }
    let mut cloned = x.clone();

    let mut reverted_number = 0;
    while cloned > reverted_number {
        reverted_number = reverted_number * 10 + cloned % 10;
        cloned /= 10;
    }
    return cloned == reverted_number || cloned == reverted_number / 10;
}

#[cfg(test)]
mod tests {
    use super::is_palindrome;
    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(121));
        assert!(is_palindrome(1221));
        assert!(!is_palindrome(122));
    }
}
