// https://atcoder.jp/contests/abc238/tasks/abc238_d

use itertools::Itertools;

fn solve(t: usize, a_s: Vec<u64, u64)>) -> Vec<bool> {
    let mut result = Vec<bool>::new();
    a_s.iter()
        .map(|&(a, s)| if 2 * a <= s && (s - 2 * a) & a == 0 {
            result.push(true);
        } else {
            result.push(false);
        });
    result
}


#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(vec![true, false], solve(2, vec![(1, 8), (4, 2)]));
        assert_eq!(
            vec![false, true, true, false],
            solve(
                4,
                vec![
                    (201408139683277485, 381410962404666524), (360288799186493714, 788806911317182736),
                    (18999951915747344, 451273909320288229), (962424162689761932, 1097438793187620758),
                ]
            )
        );
    }
}