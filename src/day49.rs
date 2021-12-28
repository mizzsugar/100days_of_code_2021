// https://atcoder.jp/contests/typical90/tasks/typical90_p

fn solve(n: i64, a: i64, b: i64, c: i64) -> i64 {
    let mut ans = std::i64::MAX;

    for i in 0..9999 {
        for j in 0..9999 - i {
            let v = n - (a * i) - (b * j);
            if v < 0 || v % c != 0 {
                continue;
            }
            let r = i + j + v / c;
            ans = ans.min(r);
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        let actual = solve(227, 21, 47, 56);
        let expected = 5;
        assert_eq!(actual, expected);
    }
}
