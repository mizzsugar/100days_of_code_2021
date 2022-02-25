// https://atcoder.jp/contests/abc234/tasks/abc234_c


fn solve(k: u64) {
    let ans = format!("{:b}", k)
        .chars()
        .map(|c| if c == '1' { '2' } else { '0' })
        .collect::<String>();
    ans
}


#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(22, solve(3));
        assert_eq!(2022, solve(11));
    }
}
