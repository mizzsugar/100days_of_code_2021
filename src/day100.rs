// https://atcoder.jp/contests/abc232/tasks/abc232_c


fn solve(n: usize, m: usize, ab: Vec<(usize, usize)>, cd: Vec<(usize, usize)>) -> bool{
    let mut taka = vec![vec![0; n]; n];
    let mut aoki = vec![vec![0; n]; n];
    for (a, b) in ab {
        taka[a - 1][b - 1] = 1;
        taka[b - 1][a - 1] = 1;
    }
    for (c, d) in cd {
        aoki[c - 1][d - 1] = 1;
        aoki[d - 1][c - 1] = 1;
    }
    let same = (0..n).permutations(n).any(|perm| {
        for i in 0..n {
            for j in 0..n {
                if taka[i][j] != aoki[perm[i]][perm[j]] {
                    return false
                }
            }
        }
        true
    });
    same
}


#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert!(solve(4, 4, vec![(1, 2), (1, 3), (1, 4), (3, 4)], vec![(1, 3), (1, 4), (2, 3), (3, 4)]));
        assert!(solve(8, 0, vec![], vec![]));
    }
}
