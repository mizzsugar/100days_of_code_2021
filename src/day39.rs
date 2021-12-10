// https://atcoder.jp/contests/abc085/tasks/abc085_b

fn solve(mut nums: Vec<u32>) -> u32 {
    nums.sort();
    nums.dedup();
    nums.len() as u32
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(3, solve(vec![10, 8, 8, 6]));
        assert_eq!(1, solve(vec![15, 15, 15]));
        assert_eq!(4, solve(vec![50, 30, 50, 100, 50, 80, 30]));
    }
}
