// https://atcoder.jp/contests/typical90/tasks/typical90_a


fn solve(n:usize, l:i32, k:i32, mut a: Vec<i32>) -> i32{
    a.push(l);
    let mut ac = 0;
    let mut wa = l;
    while wa-ac > 1 {
        let mid = (ac+wa)/2;
        let mut count = 0;
        let mut x = 0;
        for i in 0..(n+1) {
            if a[i]-x >= mid {
                count+=1;
                x = a[i];
            }
        }
        if count >= k+1 {
            ac = mid;
        } else {
            wa = mid;
        }
    }
    ac
}


#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(13, solve(3, 34, 1, vec![8, 13, 26]));
        assert_eq!(12, solve(7, 45, 2, vec![7, 11, 16, 20, 28, 34, 38]));
    }
}
