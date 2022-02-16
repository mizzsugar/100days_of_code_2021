// https://atcoder.jp/contests/abc238/tasks/abc238_c


fn solve(n: u64) -> u64 {
    const MOD: u64 = 998244353;
    let nc2 = |n| n % MOD * ((n + 1) % MOD) / 2 % MOD;
    (0..).map(|i| 10u64.pow(i)).take_while(|&k| k <= n).map(|k| nc2((n + 1).min(k * 10) - k)).sum::<u64>()% MOD
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(73, solve(16));
        assert_eq!(13870, solve(238));
    }
}