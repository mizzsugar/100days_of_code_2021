// https://atcoder.jp/contests/typical90/tasks/typical90_ca

fn solve(mut a: Vec<Vec<i64>>, b: Vec<Vec<i64>>) -> i64 {
    let h = a.len();
    let w = b.len();
    for i in 0..h {
        for j in 0..w {
            a[i][j] -= b[i][j];
        }
    }

    let mut res = 0_i64;
    for i in 0..h - 1 {
        for j in 0..w - 1 {
            let cur = a[i][j];
            a[i][j] -= cur;
            a[i][j + 1] -= cur;
            a[i + 1][j] -= cur;
            a[i + 1][j + 1] -= cur;
            res += cur.abs();
        }
    }

    if a[h - 2][w - 1] == 0 && a[h - 1][w - 2] == 0 && a[h - 1][w - 1] == 0 {
        return res
    }
    return -1
}


#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(
            -1,
            solve(
                vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
                vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]],
            )
        );
        assert_eq!(
            140,
            solve(
                vec![vec![6, 17, 18, 29, 22], vec![39, 50, 25, 39, 25], vec![34, 34, 8, 25, 17], vec![28, 48, 25, 47, 42], vec![27, 47, 24, 32, 28]],
                vec![vec![4, 6, 3, 29, 28], vec![48, 50, 21, 48, 29], vec![44, 44, 19, 47, 28], vec![4, 49, 46, 29, 28], vec![4, 49, 45, 1, 1]],
            )
        );
    }
}
