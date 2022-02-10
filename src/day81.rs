// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_e


fn solve(n: usize, x: usize, y: usize) -> usize{
    (1..=n).filter(|i| i % x == 0 || i % y == 0).count()
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(7, solve(15, 3, 5));
        assert_eq!(160839, solve(1000000, 11, 13));
    }
}
