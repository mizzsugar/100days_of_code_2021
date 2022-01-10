// https://atcoder.jp/contests/typical90/tasks/typical90_av

fn solve(n: usize, k: usize, abs: Vec<(u32, u32)>) -> u32 {
    let mut x = vec![];
    for &(a, b) in &abs {
        x.push(b);
        x.push(a - b);
    }
    x.sort();
    x.reverse();
    let mut ans = 0;
    for i in 0..k {
        ans += x[i];
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(21, solve(4, 3, vec![(4, 3), (9, 5), (15, 8), (8, 6)]));
    }
}