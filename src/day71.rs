// https://atcoder.jp/contests/typical90/tasks/typical90_cf


fn solve(n: usize, s: String) -> usize {
    let s: Vec<char> = s.chars().collect();
    let mut a = vec![0; n+1];
    let mut b = vec![0; n+1];
    for i in 1..=n {
        match s[i-1] {
            'o' => {
                a[i] = i;
                b[i] = b[i-1];
            },
            'x' => {
                a[i] = a[i-1];
                b[i] = i;
            },
            _ => unreachable!(),
        }
    }
    let mut ans = 0;
    for i in 1..=n {
        ans += a[i].min(b[i]);
    }
    ans
}


#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(5, solve(4, String::from("ooxo")));
        assert_eq!(10, solve(5, String::from("oxoxo")));
    }
}
