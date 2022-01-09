// https://atcoder.jp/contests/typical90/tasks/typical90_at

use std::collections::HashMap;


fn solve(n: usize, a: Vec<usize>, b: Vec<usize>, c: Vec<usize>) -> usize {
    let mut ah:HashMap<usize,usize> = HashMap::new();
    let mut bh:HashMap<usize,usize> = HashMap::new();
    let mut ch:HashMap<usize,usize> = HashMap::new();
    for i in 0..n {
        let x = ah.entry(a[i]%46).or_insert(0);
        *x += 1;
        let y = bh.entry(b[i]%46).or_insert(0);
        *y += 1;
        let z = ch.entry(c[i]%46).or_insert(0);
        *z += 1;
    }

    let mut ans = 0;
    for i in 0..46 {
        for j in 0..46 {
            for k in 0..46 {
                if (i + j + k) % 46 == 0 {
                    ans += 
                        *ah.entry(i).or_insert(0) 
                        * *bh.entry(j).or_insert(0) 
                        * *ch.entry(k).or_insert(0) 
                }
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(3, solve(3, vec![10, 13, 93], vec![5, 27, 35], vec![55, 28, 52]));
        assert_eq!(27, solve(3, vec![10, 56, 102], vec![16, 62, 108], vec![20, 66, 112]));
    }
}
