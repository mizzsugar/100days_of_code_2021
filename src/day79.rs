// https://atcoder.jp/contests/typical90/tasks/typical90_aq

fn solve(h: usize, w: usize, start: (usize, usize), goal: (usize, usize), mut s: Vec<char>) -> usize {
    let mut queue = VecDeque::<(usize, (usize, usize))>::new();
    let mut map = vec![vec![[std::u64::MAX; 4]; w]; h];
    for i in 0..4 {
        queue.push_back((i, start));
        map[start.0][start.1][i] = 0;
    }
    while let Some((direction, (x, y))) = queue.pop_front() {
        for &(direction1, (x1, y1)) in [
            (0, (x.saturating_sub(1), y)),
            (1, (x + 1, y)),
            (2, (x, y.saturating_sub(1))),
            (3, (x, y + 1)),
        ]
        .iter()
        {
            let cost = map[x][y][direction] + if direction1 != direction { 1 } else { 0 };
            if (0..h).contains(&x1)
                && (0..w).contains(&y1)
                && s[x1][y1] == '.'
                && map[x1][y1][direction1] > cost
            {
                map[x1][y1][direction1] = cost;
                if direction == direction1 {
                    queue.push_front((direction1, (x1, y1)));
                } else {
                    queue.push_back((direction1, (x1, y1)));
                }
            }
        }
    }
    map[goal.0][goal.1].iter().min().unwrap()
}


#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(2, solve(3, 3, (1, 1), (3, 3), vec!['..#', '#.#', '#..']));
    }
}
