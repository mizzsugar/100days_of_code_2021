// https://atcoder.jp/contests/abc236/tasks/abc236_b


fn solve(n: usize, t: Vec<usize>) -> u32 {
    let mut ans: Vec<u32> = vec![0; n];
 
    for num in &t {
        ans[num-1] += 1;
    }
 
    for i in 0..n {
        if ans[i] == 3 {
            return i + 1
        }
    }
}


#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(3, solve(3, vec![1, 3, 2, 3, 3, 2, 2, 1, 1, 1, 2]));
        assert_eq!(2, solve(4, vec![3, 2, 1, 1, 2, 4, 4, 4, 4, 3, 1, 3, 2, 1, 3]));
    }
}
