// https://atcoder.jp/contests/abc234/tasks/abc234_d


use std::collections::BinaryHeap;


fn solve(n: usize, k: usize, l: Vec<i32>) {
    let mut h = BinaryHeap::new();
    let mut result = Vec<i32>::new();
    let mut ans = 0;
    for i in 0..n {
        h.push(-l[i]);
        if i >= k - 1 {
            ans = ans.max(-h.pop().unwrap());
            result.push(ans);
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(vec![1, 2], solve(3, 2, vec![1, 2, 3]));
        assert_eq!(vec![2, 3, 3, 5, 6, 7, 7], solve(11, 5, vec![3, 7, 2, 5, 11, 6, 1, 9, 8, 10, 4]));
    }
}
