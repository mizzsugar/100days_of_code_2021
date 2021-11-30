// https://leetcode.com/problems/factorial-trailing-zeroes
// https://leetcode.com/problems/factorial-trailing-zeroes/discuss/235531/My-Rust-solution-in-0ms

use std::collections::HashSet;

fn trailing_zeroes(n: i32) -> i32 {
    fn helper(n: &i32) -> i32 {
        if *n == 0 {
            0
        } else {
            n / 5 + helper(&(n / 5))
        }
    }
    let ret = helper(&n);
    ret
}

#[cfg(test)]
mod tests {
    use super::trailing_zeroes;

    #[test]
    fn test_trailing_zeroes() {
        assert_eq!(0, trailing_zeroes(3));
        assert_ne!(0, trailing_zeroes(5));
    }
}
