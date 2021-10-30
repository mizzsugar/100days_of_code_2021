// https://leetcode.com/problems/longest-common-prefix

use std::u32;
use substring::Substring;

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 0 {
        return String::from("");
    }
    let mut min_len = std::u32::MAX;
    for s in &strs {
        let s_len = s.len() as u32;
        min_len = std::cmp::min(min_len, s_len);
    }
    let mut low = 1;
    let mut high = min_len;
    while low <= high {
        let middle = (low + high) / 2;
        if is_common_prefix(&strs, middle) {
            low = middle + 1;
        } else {
            high = middle - 1;
        }
    }

    let pop = (low + high) / 2;
    // u32 to usize
    // https://stackoverflow.com/questions/43704758/how-to-idiomatically-convert-between-u32-and-usize

    // substring
    // https://stackoverflow.com/questions/37157926/is-there-a-method-like-javascripts-substr-in-rust/37158376
    let res = strs[0].substring(0, pop as usize - 1);
    String::from(res)
}

fn is_common_prefix(strs: &Vec<String>, len: u32) -> bool {
    let str1: String = strs[0].chars().skip(len as usize).take(0).collect();
    let mut i = 1;
    while i < strs.len() {
        if !strs[i].starts_with(&str1) {
            return false;
        }
        i += 1;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::longest_common_prefix;
    #[test]
    fn test_longest_common_prefix() {
        let strs = vec![
            String::from("leets"),
            String::from("leetscode"),
            String::from("leet"),
            String::from("leeds"),
        ];
        let expected = String::from("lee");
        let actual = longest_common_prefix(strs);
        assert_eq!(expected, actual);
    }
}
