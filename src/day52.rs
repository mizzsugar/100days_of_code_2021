// https://atcoder.jp/contests/typical90/tasks/typical90_v


fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

fn solve(a: usize, b: usize, c: usize) -> usize {
    let d = gcd(a, gcd(b, c));
    (a + b + c) / d - 3
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(4, solve(2, 2, 3));
        assert_eq!(1, solve(2, 2, 4));
    }
}
