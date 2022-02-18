// https://atcoder.jp/contests/abc237/tasks/abc237_b

fn solve(h: usize,  w: usize, a: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut result = [[0;h];w];
    for i in 0..w {
        for j in 0..h {
            result[j][i] = a[j][i];
        }
    }
    return result as Vec<Vec<i64>>
}


#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        let expected = vec![vec![1, 4, 7, 10], vec![2, 5, 8, 11], vec![3, 6, 9, 12]];
        let before = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10, 11, 12]];
        assert_eq!(expected, solve(4, 3, nums));
    }
}
