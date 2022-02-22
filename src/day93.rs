// https://atcoder.jp/contests/abc235/tasks/abc235_b


fn solve(n:i32, arr: Vec<i32>) -> i32 {
    let mut pre = -1;
    for i in arr {
        if i > pre {
            pre = i;
        } else {
            break;
        }
    }
     pre
}


#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(10, solve(5, vec![1, 5, 10, 4, 2]));
        assert_eq!(100000, solve(3, vec![100, 1000, 100000]));
    }
}
