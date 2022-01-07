// https://atcoder.jp/contests/typical90/tasks/typical90_ar

fn solve(n: usize, q: usize, mut a: Vec<usize>, nums: Vec<(i8, usize, usize)>) -> usize {
    let mut d = 0;
    let mut result = 0;
    for (t, x, y) in nums {
        let x = (n + x - 1 + d) % n;
        let y = (n + y - 1 + d) % n;
        match t {
            1 => a.swap(x, y),
            2 => d = (n + d - 1) % n,
            3 => result = a[x],
            _ => unreachable!(),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(
            18,
            solve(
                9,
                6,
                vec![16, 7, 10, 2, 9, 18, 15, 20, 5],
                vec![
                    (2, 0, 0),
                    (1, 1, 4),
                    (2, 0, 0),
                    (1, 8, 5),
                    (2, 0, 0),
                    (3, 6, 0),
                ]
            )
        );
    }
}
