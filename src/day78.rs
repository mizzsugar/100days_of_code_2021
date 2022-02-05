// https://atcoder.jp/contests/typical90/tasks/typical90_ap

fn solve(k: usize) -> usize {
    let mut ans = 0;
    
    if k % 9 == 0 {
        let mut dp = vec![0; k+1];
        dp[0] = 1;
        for i in 1..=k {
            for j in 1..=9.min(i) {
                dp[i] += dp[i - j];
                dp[i] %= MOD;
            }
        }
        ans = dp[k];
    }
 
    ans
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(0, solve(1));
        assert_eq!(757186539, solve(234));
    }
}
