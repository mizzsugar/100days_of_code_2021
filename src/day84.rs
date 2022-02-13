// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_k

fn solve(n: usize) -> Vec<usize> {
    let mut result = vec![];
    for i in 2..=n {
        let mut prime = true;
        for j in 2..i {
            if i % j == 0 {
                prime = false;
                break;
            }
        }
 
        if prime {
            result.push(i);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(vec![2, 3, 5, 7], solve(10));
    }
}
