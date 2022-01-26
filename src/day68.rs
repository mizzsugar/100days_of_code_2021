// https://atcoder.jp/contests/typical90/tasks/typical90_bz


fn solve(n: usize, ab: Vec<(usize, usize)>) -> usize {
    let mut min_degree = vec![0; n];
    for (a,b) in ab {
        min_degree[a.max(b)] += 1;
    }
    min_degree.iter().filter(|&&i| i == 1).count()
}


#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(3, solve(6, vec![(5, 5), (1, 2), (1, 3), (3, 2), (5, 2), (4, 2)]));
        assert_eq!(1, solve(2, vec![(2, 1), (1, 2)]));
    }
}