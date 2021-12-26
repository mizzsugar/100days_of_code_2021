// https://atcoder.jp/contests/typical90/tasks/typical90_d

use std::i32::MAX;

fn solve(mut a: Vec<i32>, mut b: Vec<i32>) -> Vec<i32> {
    let n = a.len();
    let q = b.len();
    a.sort();
    let mut result: Vec<i32> = Vec::new();
    for j in 0..q {
        let i = a.binary_search(&b[j]).unwrap_or_else(|x| x);
        let mut d1 = MAX;
        let mut d2 = MAX;
        if i < n {
            d1 = (b[j] - a[i]).abs()
        }
        if 0 < i {
            d2 = (b[j] - a[i - 1]).abs();
        }
        result.push(d1.min(d2));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        let actual = solve(vec![4000, 4400, 5000, 3200], vec![3312, 2992, 4229]);
        let expected = vec![112, 208, 171];
        assert_eq!(actual, expected);
    }
}
