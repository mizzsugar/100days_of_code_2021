// https://leetcode.com/problems/power-of-three

fn is_power_of_three(n: i32) -> bool {
    let mut n = n;
    if n > 1 {
        while n % 3 == 0 {
            n /= 3;
        }
    }
    n == 1
}

#[cfg(test)]
mod tests {
    use super::is_power_of_three;

    #[test]
    fn test_is_power_of_three() {
        assert!(is_power_of_three(27));
        assert!(!is_power_of_three(45));
    }
}
