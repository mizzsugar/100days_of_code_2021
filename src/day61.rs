// https://atcoder.jp/contests/typical90/tasks/typical90_ax

fn solve(n: usize, l: usize) -> usize {
    let _mod: usize = 1_000_000_007;
	let mut dp = vec![0; n + 1];
	dp[0] = 1;
	for i in 1..=n {
		dp[i] += dp[i - 1];
		dp[i] %= _mod;
		if i >= l {
			dp[i] += dp[i - l];
			dp[i] %= _mod;
		}
	}
    dp[n]
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(3, solve(3, 2));
        assert_eq!(2, solve(4, 4));
    }
}