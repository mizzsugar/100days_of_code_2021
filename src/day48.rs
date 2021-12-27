// https://atcoder.jp/contests/typical90/tasks/typical90_n

fn solve(mut a: Vec<i32>, mut b: Vec<i32>) -> i32 {
    a.sort();
    b.sort();

    let mut sum_diff_abs: i32 = 0;
    for i in 0..a.len() {
        sum_diff_abs += (a[i as usize] - b[i as usize]).abs();
    }

    sum_diff_abs
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        let actual = solve(vec![8, 6, 9, 1, 2, 0], vec![1, 5, 7, 2, 3, 9]);
        let expected = 5;
        assert_eq!(actual, expected);
    }
}
