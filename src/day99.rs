// https://atcoder.jp/contests/abc232/tasks/abc232_b


use std::collections::HashMap;


fn solve(s: String, t: String) -> bool {
    let v: Vec<i8> = s.chars().zip(t.chars()).map(|(a, b)| {
        let x = b as i8 - a as i8;
        if x >= 0 {
            x
        } else {
            x + 26
        }
    }).collect();
 
    if v.iter().all(|&x| x == v[0]) {
        return true
    }
    false
}


#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert!(solve(String::from("abc"), String::from("ijk")));
        assert!(!solve(String::from("ppq"), String::from("qqp")));
    }
}
