// https://atcoder.jp/contests/abc086/tasks/arc089_a
// https://qiita.com/tubo28/items/e6076e9040da57368845#%E7%AC%AC-10-%E5%95%8F-abc-086-c---traveling-300-%E7%82%B9

fn solve(mut v: Vec<(i32, i32, i32)>) -> bool {
    v.insert(0, (0, 0, 0));
    v[..].windows(2).all(|w| {
        let (t, x, y) = w[0];
        let (nt, nx, ny) = w[1];
        let time = nt - t;
        let dist = (nx -x).abs() + (ny - y).abs();
        dist <= time && time % 2 == dist % 2
    })
}


#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert!(solve(vec![(3, 1, 2), (6, 1, 2)]));
        assert!(!solve(vec![(2, 100, 100)]));
        assert!(!solve(vec![(5, 1, 1), (100, 1, 1)]));
    }
}
