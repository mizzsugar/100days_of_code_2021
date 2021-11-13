// https://leetcode.com/problems/reverse-integer
// https://leetcode.com/problems/reverse-integer/discuss/680455/Rust-0ms-100

pub fn reverse(x: i32) -> i32 {
    fn helper(mut n: i32) -> Option<i32> {
        let mut res = 0i32;
        while n.abs() > 0 {
            res = res.checked_mul(10)?.checked_add(n % 10)?;
            n /= 10;
        }
        Some(res)
    }
    helper(x).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::reverse;
    #[test]
    fn test_reverse() {
        let num = -321;
        let expectd = -123;
        assert_eq!(expectd, reverse(num));
    }

    #[test]
    fn test_reverse_zero() {
        let num = 0;
        let expectd = 0;
        assert_eq!(expectd, reverse(num));
    }
}
