// https://leetcode.com/problems/powx-n
// https://leetcode.com/problems/powx-n/discuss/931844/Rust%3A-recursive-solution

fn my_pow(x: f64, n: i32) -> f64 {
    fn pow(x: f64, res: f64, n: i64) -> f64 {
        match n {
            0 => res,
            n if n & 1 == 1 => pow(x * x, res * x, n >> 1),
            _ => pow(x * x, res, n >> 1),
        }
    }

    match n {
        0 => 1.0,
        n if n < 0 => pow(1.0 / x, 1.0, (n as i64).abs()),
        _ => pow(x, 1.0, n as i64),
    }
}

#[cfg(test)]
mod tests {
    use super::my_pow;

    #[test]
    fn test_my_pow() {
        assert_eq!(1024_f64, my_pow(2_f64, 10));
        assert_eq!(0.25_f64, my_pow(2_f64, -2));
    }
}
