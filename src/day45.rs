// https://atcoder.jp/contests/typical90/tasks/typical90_d

fn solve(h: usize, w: usize, ll: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut yoko_sum = vec![0; h];
    let mut tate_sum = vec![0; w];
    for i in 0..h {
        for j in 0..w {
            yoko_sum[i] += ll[i][j];
            tate_sum[j] += ll[i][j];
        }
    }

    let mut s: Vec<Vec<i32>> = vec![vec![0; w]; h];

    for i in 0..h {
        for j in 0..w {
            s[i][j] = yoko_sum[i] + tate_sum[j] - ll[i][j];
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        let actual = solve(3, 3, vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]]);
        let expected = vec![vec![5, 5, 5], vec![5, 5, 5], vec![5, 5, 5]];
        assert_eq!(actual, expected);
    }
}
