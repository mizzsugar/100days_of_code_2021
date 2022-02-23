// https://atcoder.jp/contests/abc235/tasks/abc235_d

use std::collections::VecDeque;


fn solve(a: usize, n: usize) -> i32 {
    let mut dist = vec![-1; 1000000];
    let mut q = VecDeque::new();
    q.push_back(1);
    dist[1] = 0;
 
    while let Some(x) = q.pop_front() {
        if x * a < 1000000 {
            let y = x * a;
            if dist[y] == -1 {
                dist[y] = dist[x] + 1;
                q.push_back(y);
            }
        }
        if x >= 10 && x % 10 != 0 {
            let y = format!("{}{}", x % 10, x / 10).parse::<usize>().unwrap();
            if dist[y] == -1 {
                dist[y] = dist[x] + 1;
                q.push_back(y);
            }
        }
    }
 
    dist[n]
}


#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(4, solve(3, 72));
        assert_eq!(-1, solve(2 5));
    }
}
