// https://atcoder.jp/contests/abc236/tasks/abc236_c

use std::collections::HashSet;


fn solve(n: usize, m: usize, s: Vec<String>, t: Vec<String>) -> Vec<bool> {
    let mut hs = HashSet::new();
    let mut result = Vec<bool>::new();

    for i in 0..m {
        hs.insert(t[i].to_owned());
    }
    for i in 0..n {
        if hs.contains(&s[i]) {
            result.push(true);
        } else {
            result.push(false);
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(
            vec![true, false, true, false, true],
            solve(
                5, 3,
                vec![String::from("tokyo"), String::from("kanda"), String::from("akiba"), String::from("okachi"), String::from("ueno")],
                vec![String::from("tokyo"), String::from("akiba"), String::from("ueno")],
            )
        );
    }
}
