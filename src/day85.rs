// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_l

fn solve(n: usize) -> bool {
    let mut result : bool = true;
    for i in 2..n+1{
        if i * i > n{
            break;
        }
        if n % i == 0{
            result = false;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert!(solve(7));
        assert!(!solve(8));
    }
}
