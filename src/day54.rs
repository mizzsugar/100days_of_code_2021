// https://atcoder.jp/contests/typical90/tasks/typical90_aa
use itertools::Itertools;

const INF: usize = 1 << 30;

fn solve(n: usize, a: Vec<Vec<usize>>, m: i32, xy: Vec<(usize,usize)>) -> i32 {
    let mut result: i32 = -1;
 
    let mut passable = vec![vec![true; n]; n];

    for (x_i, y_i) in xy {
        passable[x_i-1][y_i-1] = false;
        passable[y_i-1][x_i-1] = false;
    }

    for p in (0..n).permutations(n) {
        let mut available = true;
        for i in 0 .. n - 1 {
            let x = p[i];
            let y = p[i+1];
 
            if !passable[x][y] {
                available = false;
            }
        }
 
        if available {
            let mut time: i32 = 0;
 
            for i in 0 .. n {
                time += a[p[i]][i] as i32;
            }
 
            if result < 0 || time < result {
                result = time;
            }
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        let actual = solve(
            3, vec![vec![1, 10, 100], vec![10, 1, 100], vec![100, 10, 1]], 1, vec![(1, 2)]
        );
        let expected = 111;
        assert_eq!(actual, expected);
    }
}
