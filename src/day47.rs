// https://atcoder.jp/contests/typical90/tasks/typical90_j

fn solve(cp: Vec<(usize, usize)>, lr: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut a: usize = 0;
    let mut b: usize = 0;

    let mut g_1: Vec<usize> = Vec::new();
    g_1.push(0);
    let mut g_2: Vec<usize> = Vec::new();
    g_2.push(0);

    let mut result: Vec<(usize, usize)> = Vec::new();
    for (c, p) in cp {
        match c {
            1 => {
                a += p;
            }
            _ => {
                b += p;
            }
        }
        g_1.push(a);
        g_2.push(b);
    }

    for (l, r) in lr {
        result.push((g_1[r] - g_1[l - 1], g_2[r] - g_2[l - 1]));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        let actual = solve(
            vec![
                (1, 72),
                (2, 78),
                (2, 94),
                (1, 23),
                (2, 89),
                (1, 40),
                (1, 75),
            ],
            vec![(2, 6)],
        );
        let expected = vec![(63, 261)];
        assert_eq!(actual, expected);
    }
}
