// https://atcoder.jp/contests/abc233/tasks/abc233_b


use std::iter::FromIterator;


fn solve(mut l: usize, mut r: usize, s: String) -> String {
    let mut cs: Vec<char> = s.chars().collect();
    cs[l-1..r].reverse();
    String::from_iter(cs)
}


#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(String::from("abgfedch"), solve(3, 7, String::from("abgfedch")));
        assert_eq!(String::from("reviver"), solve(1, 7, String::from("reviver")));
    }
}
