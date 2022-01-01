// https://atcoder.jp/contests/typical90/tasks/typical90_x


fn solve(n: i32, k: i32, a: Vec<i32>, b: Vec<i32>) -> bool {
    let diff: Vec<i32> = a.iter().zip(b.iter()).map(|p| (p.0 - p.1).abs() ).collect();
    let remaining_k = k - diff.iter().sum::<i32>();
    if remaining_k < 0 {
      return false;
    }
    return remaining_k % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert!(solve(2, 5, vec![1, 3], vec![2, 1]));
        assert!(!solve(3, 1, vec![7, 8, 9], vec![7, 8, 9]));
    }
}
