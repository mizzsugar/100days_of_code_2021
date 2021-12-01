// https://leetcode.com/problems/excel-sheet-column-number/
// https://leetcode.com/problems/excel-sheet-column-number/discuss/357060/One-line-solution-in-Rust

use std::collections::HashSet;

fn title_to_number(s: String) -> i32 {
    s.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| (((c as u8 - 64) % 27) as i32 * 26_i32.pow(i as u32)))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::title_to_number;

    #[test]
    fn test_title_to_number() {
        assert_eq!(1, title_to_number(String::from("A")));
        assert_eq!(2147483647, title_to_number(String::from("FXSHRXW")));
    }
}
