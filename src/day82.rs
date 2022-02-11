// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_i


fn solve(n: usize, s: usize, a: Vec<usize>) -> bool {
    let mut dp = vec![0; s + 1];
    dp[0] = 1;
    for &i in a.iter() {
        for j in (i..=s).rev() {
            dp[j] |= dp[j - i];
            if dp[s] == 1 {
                return true
            }
        }
    }
    false
}


#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert!(solve(3, 11, vec![2, 5, 9]));
        assert!(!solve(4, 11, vec![3, 1, 4, 5]));
    }
}
