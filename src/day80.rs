// https://atcoder.jp/contests/typical90/tasks/typical90_bf

use std::collections::HashMap;


fn solve(mut n: usize, k: usize) -> usize{
    let mut map = HashMap::new();
    let mut stack = Vec<usize>::new()
    let mut ans_idx = k - 1;
 
    for i in 0..k {
        n = next_num(n);
        if map.contains_key(&n) {
            let loop_start = map[&n];
            let loop_size = i - loop_start;
            ans_idx = (k - loop_start - 1) % loop_size + loop_start;
            break;
        } else {
            map.insert(n, i);
            stack.push(n);
        }
    }
    stack[ans_idx]
}

fn next_num(mut n: usize) -> usize {
    let mut nxt = n;
    while n != 0 {
        nxt += n % 10;
        n /= 10;
    }
    return nxt % 100000;
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(13, solve(5, 3));
        assert_eq!(0, solve(0, 100));
    }
}
