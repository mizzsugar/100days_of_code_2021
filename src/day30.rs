// https://leetcode.com/problems/happy-number

use std::collections::HashSet;

fn get_next(n: i32) -> i32 {
    let mut n = n;
    let mut total_sum = 0;
    while n > 0 {
        let d = n % 10;
        n /= 10;
        total_sum += d * d;
    }
    total_sum
}

fn is_happy(n: i32) -> bool {
    let mut seen: HashSet<i32> = HashSet::new();
    let mut m = n;
    while m != 1 && !seen.contains(&m) {
        seen.insert(m);
        m = get_next(m);
    }
    return m == 1;
}

#[cfg(test)]
mod tests {
    use super::is_happy;

    #[test]
    fn test_is_happy() {
        assert!(is_happy(19));
        assert!(!is_happy(20));
    }
}
