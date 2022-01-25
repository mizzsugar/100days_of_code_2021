// https://atcoder.jp/contests/typical90/tasks/typical90_bx

fn solve(a: Vec<u64>) -> bool {
    let n  = a.len();
    let sum = a.iter().sum::<u64>();
    if sum % 10 != 0 {
        return false
    }

    let k = sum / 10;
 
    for &k in &[k, 9 * k] {
        let mut j = 0;
        let mut sum = 0;
        for i in 0..n {
            while j < n && (sum + a[j]) <= k {
                sum += a[j];
                j += 1;
            }
            if sum == k {
                return true
            }
            if i == j {
                sum += a[j];
                j += 1;
            }
            sum -= a[i];
        }
    }
    return false
}


#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert!(solve(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]));
        assert!(!solve(vec![1, 1, 1]));
        assert!(solve(vec![1, 18, 1]));
    }
}