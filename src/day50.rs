// https://atcoder.jp/contests/typical90/tasks/typical90_r

use std::f64::consts::PI;

fn solve(t: f64, l: f64, x: f64, y: f64, e: Vec<f64>) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();
    for i in e {
        let px = 0.0;
        let py = -l * (2.0 * PI * i / t).sin() / 2.0;
        let pz = l / 2.0 - l / 2.0 * (2.0 * PI * i / t).cos();
        let dist = ((px - x) * (px - x) + (py - y) * (py - y)).sqrt();
        let angle = pz.atan2(dist) * 180.0 / PI;
        result.push(angle);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        let actual = solve(4_f64, 2_f64, 1_f64, 1_f64, vec![0_f64, 1_f64, 2_f64, 3_f64]);
        let expected = vec![
            0.000000000000,
            24.094842552111,
            54.735610317245,
            45.000000000000,
        ];
        assert_eq!(actual, expected);
    }
}
