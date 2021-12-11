// https://atcoder.jp/contests/abc085/tasks/abc085_c
// https://qiita.com/tubo28/items/e6076e9040da57368845#%E7%AC%AC-8-%E5%95%8F-abc-085-c---otoshidama-300-%E7%82%B9

fn solve(n: i32, x: i32) -> (i32, i32, i32) {
    let mut ans = None;
    'outer: for i in 0..n+1 {
        for j in 0..n - i + 1 {
            let k = n - i -j;
            if i * 1000 + j * 5000 + k * 1000 == x {
                ans = Some((i, j, k));
                break 'outer;
            }
        }
    }
    let (x, y, z) = ans.unwrap_or((-1, -1, -1));
    return (x, y, z)
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!((4, 0, 5), solve(9, 45000));
        assert_eq!((-1, -1, -1), solve(20, 196000));
        assert_eq!((14, 27, 959), solve(1000, 123400));
    }
}
