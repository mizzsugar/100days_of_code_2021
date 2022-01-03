// https://atcoder.jp/contests/typical90/tasks/typical90_ag


fn solve(h: usize, w: usize) -> usize {
    if h == 1 || w == 1 {
       return h * w
    }
    let a = if h % 2 == 0 { h / 2 } else { (h + 1) / 2 };
    let b = if w % 2 == 0 { w / 2 } else { (w + 1) / 2 };
    a * b
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(2, solve(2, 3));
        assert_eq!(4, solve(3, 4));
    }
}
