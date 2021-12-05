// https://atcoder.jp/contests/abc081/tasks/abc081_b
// https://qiita.com/tubo28/items/e6076e9040da57368845#%E7%AC%AC-3-%E5%95%8F-abc-081-b---shift-only-200-%E7%82%B9

fn solve(s: Vec<u32>) -> u32 {
    s.iter().map(|&x| x.trailing_zeros()).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(2, solve(vec![8, 12, 40]));
        assert_eq!(0, solve(vec![5, 6, 8, 10]));
        assert_eq!(
            8,
            solve(vec![
                382253568, 723152896, 37802240, 379425024, 404894720, 471526144
            ])
        );
    }
}
