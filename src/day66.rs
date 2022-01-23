// https://atcoder.jp/contests/typical90/tasks/typical90_bw


fn solve(n: u64) -> u64 {
    let mut n_pf = 1;
    for i in 2..((n as f64).sqrt() + 1.) as u64 {
      if n % i == 0 {
        while n > i && n % i == 0 {
            n_pf += 1;
            n /= i;
        }
      }
    }
    (n_pf as f64).log2().ceil() as u64
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(2, solve(42));
        assert_eq!(3, solve(48));
    }
}
