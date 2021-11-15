// https://leetcode.com/problems/valid-anagram

pub fn is_anagram(s: String, t: String) -> bool {
    let mut s_char: Vec<char> = s.chars().collect();
    let mut t_char: Vec<char> = t.chars().collect();
    s_char.sort();
    t_char.sort();
    s_char == t_char
}

#[cfg(test)]
mod tests {
    use super::is_anagram;
    #[test]
    fn test_is_anagram() {
        let s = String::from("anagram");
        let t = String::from("nagaram");
        assert!(is_anagram(s, t));
    }
}
