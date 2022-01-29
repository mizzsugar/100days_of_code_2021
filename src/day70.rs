// https://atcoder.jp/contests/typical90/tasks/typical90_cd


fn solve(mut l: u128, r: u128) -> u128 {
    let MOD = 1_000_000_000 + 7;
    let mut ans = 0;
    let mut g = l.to_string().len() as u128;

    while l <= r {
        let x = r + 1 - l;
 
        if x % 2 == 0 {
            ans += (l + r) * (x / 2) * g % MOD;
            ans %= MOD;
        } else {
            ans += (l + r) * (x / 2) * g % MOD;
            ans += (l + r) / 2 * g % MOD;
            ans %= MOD;
        }
 
        l = 10u128.pow((l.to_string().len()) as u32);
        g = 1;
    }

    ans
}


#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(12, solve(3, 5));
        assert_eq!(694, solve(98, 100));
        assert_eq!(59367733, solve(1001, 869120));
    }
}
