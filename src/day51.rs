// https://atcoder.jp/contests/typical90/tasks/typical90_t

fn solve(a: u64, b: u32, c: u64) -> bool {
    a < c.pow(b)
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert!(solve(4, 3, 2));
        assert!(!solve(8, 3, 2));
    }
}
