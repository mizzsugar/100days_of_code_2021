// https://atcoder.jp/contests/abc087/tasks/abc087_b

fn solve(a: u32, b: u32, c: u32, x: u32) -> u32 {
    let mut ans = 0;
    for i in 0..a + 1 {
        for j in 0..b + 1 {
            for k in 0..c + 1 {
                if i * 500 + j * 100 + k * 50 == x {
                    ans += 1;
                }
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(2, solve(2, 2, 2, 100));
        assert_eq!(0, solve(5, 1, 0, 150));
        assert_eq!(213, solve(30, 40, 50, 6000));
    }
}
