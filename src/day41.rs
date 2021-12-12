// https://atcoder.jp/contests/abc049/tasks/arc065_a
// https://qiita.com/tubo28/items/e6076e9040da57368845#%E7%AC%AC-8-%E5%95%8F-abc-085-c---otoshidama-300-%E7%82%B9

fn solve(x: String) -> bool {
    let patterns: Vec<Vec<char>> = ["dream", "dreamer", "erase", "eraser"]
        .iter()
        .map(|s| s.chars().rev().collect())
        .collect();

    let s: Vec<char> = x.chars().rev().collect();
    let mut s = &s[..];
    let mut successed = true;
    while s.len() > 0 {
        let matched = patterns.iter().find(|&p| s.starts_with(p));
        if let Some(p) = matched {
            s = &s[p.len()..];
        } else {
            successed = false;
            break;
        }
    }
    successed
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert!(solve(String::from("erasedream")));
        assert!(solve(String::from("dreameraser")));
        assert!(!solve(String::from("dreamerer")));
    }
}
