// https://atcoder.jp/contests/abc234/tasks/abc234_b


fn solve(n: usize, ps: Vec<<f64>>) -> f64 {
    let mut max : f64 = -1.;
    for i in 0..n {
        for j in i..n {
            let length = ((ps[i][0] - ps[j][0]).powf(2.) + (ps[i][1] - ps[j][1]).powf(2.)).sqrt();
            max = max.max(length);
        }
    }
    max
}


#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(1.4142135624, solve(3, vec![vec![0, 0], vec![0, 1], vec![1, 1]]));
        assert_eq!(1455.7159750446, solve(5, vec![vec![315, 271], vec![-2, -621], vec![-205, -511], vec![-952, 482], vec![165, 463]]));
    }
}
