// https://atcoder.jp/contests/typical90/tasks/typical90_bq

const MOD: usize = 1_000_000_007;

fn pow(x: usize, n: usize) -> usize {
    let mut res = 1;
    let mut x = x;
    let mut n = n;
    while n > 0 {
        if n % 2 == 1 {
            res *= x;
            res %= MOD;
        }
        x *= x;
        x %= MOD;
        n /= 2;
    }
    res
}

fn solve(n: usize, k: usize) -> usize {
    match (n, k) {
        (1, _) => return k,
        (_, 1) => return 0,
        (2, _) => return k * (k-1) % MOD,
        (_, _) => return k * (k-1) % MOD * pow(k-2, n-2) % MOD,
    };
}


#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(6, solve(2, 3));
        assert_eq!(0, solve(10, 2));
    }
}
