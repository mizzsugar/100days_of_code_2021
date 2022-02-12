// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_j


fn solve(n: usize) -> usize {
    factorial(n)
}

fn factorial(n: usize) -> usize {
    if n != 0 {
        return factorial(n - 1) * n
    }
    1
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(120, solve(5));
    }
}
