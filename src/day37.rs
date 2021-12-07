// https://atcoder.jp/contests/abc083/tasks/abc083_b

fn solve(n: u32, a: u32, b: u32) -> u32 {
    (1..n + 1)
        .filter(|x| {
            let sum = x
                .to_string()
                .chars()
                .map(|c| (c as u8 - b'0') as u32)
                .sum::<u32>();
            a <= sum && sum <= b
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(84, solve(20, 2, 5));
        assert_eq!(13, solve(10, 1, 2));
        assert_eq!(4554, solve(100, 4, 16));
    }
}
