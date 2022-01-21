// https://atcoder.jp/contests/typical90/tasks/typical90_bi


fn solve(a: Vec<i64>, lrv: Vec<[i64; 3]>) -> Vec<i64> {
    let n = a.len();
    let q = lrv.len();
    let mut result: Vec<i64> = Vec::new();
    let mut diff_a: Vec<i64> = vec![0; n - 1];
    let mut total_inconvenience: i64 = 0;

    for i in 0..(n - 1) {
        diff_a[i] = a[(i + 1) as usize] - a[i];
        total_inconvenience += diff_a[i].abs();
    }

    for i in 0..q {
        let l = lrv[i][0] - 1;
        let r = lrv[i][1] - 1;
        let v = lrv[i][2];
 
        if l != 0 {
            let before = diff_a[(l - 1) as usize].abs();
            diff_a[(l - 1) as usize] += v;
            let after = diff_a[(l - 1) as usize].abs();
            total_inconvenience += after - before;
        }
        if r != (n - 1) as i64 {
            let before = diff_a[r as usize].abs();
            diff_a[r as usize] -= v;
            let after = diff_a[r as usize].abs();
            total_inconvenience += after - before;
        }
 
        result.push(total_inconvenience);
    }

    result
}



#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(vec![3, 4, 4], solve(vec![1, 2, 3], vec![[2, 3, 1], [1, 2, -1], [1, 3, 2]]));
    }
}