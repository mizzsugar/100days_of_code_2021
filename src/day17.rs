// https://leetcode.com/problems/first-unique-character-in-a-string
use std::collections::HashMap;

pub fn first_uniq_char(s: String) -> i32 {
    let mut map: HashMap<&char, i32> = HashMap::new();
    let chars: Vec<char> = s.chars().collect();
    for c in &chars {
        *map.entry(c).or_insert(0) += 1;
    }

    for (index, c) in chars.iter().enumerate() {
        if map[c] == 1 {
            return index as i32;
        }
    }
    return -1;
}

#[cfg(test)]
mod tests {
    use super::first_uniq_char;
    #[test]
    fn test_first_uniq_char() {
        let s = String::from("leetcode");
        let expected: i32 = 0;
        assert_eq!(expected, first_uniq_char(s));
    }
}
