// https://atcoder.jp/contests/abc238/tasks/abc238_b


fn solve(n: usize, a: Vec<u32>) -> u32 {
    let mut mark = 0;
    let mut line = vec![];
    for ea in a {
      mark += ea;
      if mark > 360 {
        mark -= 360;
      }
      line.push(mark);
    }
    line.push(0);
    line.push(360);
    line.sort_unstable();
    let mut ans = 1;
    for i in 1..line.len() {
      let gap = line[i] - line[i-1];
      ans = std::cmp::max(ans, gap);
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(120, solve(4, vec![90, 180, 45, 195]));
        assert_eq!(359, solve(1, vec![1]));
    }
}
