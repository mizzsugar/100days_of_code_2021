// https://atcoder.jp/contests/abc081/tasks/abc081_a

fn count(s: String) -> usize {
    s.chars().filter(|&c| c == '1').count()
}

#[cfg(test)]
mod tests {
    use super::count;
    #[test]
    fn test_place() {
        let s = String::from("101");
        assert_eq!(2, count(s));
    }
}
