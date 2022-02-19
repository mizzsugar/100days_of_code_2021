// https://atcoder.jp/contests/abc237/tasks/abc237_c

fn solve(s: String) -> bool {
    let s1 = s.trim_matches(|c| c == 'a').to_string();
    let t1 = s1.chars().rev().collect::<String>();
 
    if s1 == t1 {
        let s2 = s.chars().take_while(|&c| c == 'a').count();
        let t2 = s.chars().rev().take_while(|&c| c == 'a').count();
 
        if s2 <= t2 {
            return true
        }
        return false
    }
    false
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert!(solve(String::from("kasaka")));
        assert!(!solve(String::from("atcoder")));
    }
}
