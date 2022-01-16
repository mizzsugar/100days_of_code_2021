// https://atcoder.jp/contests/typical90/tasks/typical90_az

fn solve(a: Vec<[usize; 6]>) -> usize {
    let _mod: usize = 1_000_000_007;
    let mut result = 1;
  
    for i in 0 .. a.len() {
      result = (result * a[i].iter().sum::<usize>()) % _mod;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(1421, solve(vec![[1, 2, 3, 5, 7, 11], [4, 6, 8, 9, 10, 12]]));
        assert_eq!(112, solve(vec![[11, 13, 17, 19, 23, 29]]));
    }
}