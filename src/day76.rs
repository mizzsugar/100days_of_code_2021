// https://atcoder.jp/contests/typical90/tasks/typical90_ab


fn solve(n: usize, ps: Vec<(usize, usize, usize, usize)>) {
    let mut tmp = vec![vec![0_i32; 1001]; 1001];
    let mut result = Vec<usize>::new();
 
    for p in ps{
        tmp[p.0][p.1] += 1;
        tmp[p.0][p.3] += -1;
        tmp[p.2][p.1] += -1;
        tmp[p.2][p.3] += 1;
    }
 
    for i in 0..=1000{
        let mut cnt = 0;
        for j in 0..=1000{
            cnt += tmp[i][j];
            tmp[i][j] = cnt;
        }
    }
    let mut k_cnt = vec![0_u32; n+1];
    for i in 0..=1000{
        let mut cnt = 0;
        for j in 0..=1000{
            cnt += tmp[j][i];
            tmp[j][i] = cnt;
            k_cnt[cnt as usize] += 1;
        }
    }
 
    for k in k_cnt.iter().skip(1){
        result.push(k);
    }
    result
}


#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(vec![2, 1], solve(2, vec![(1, 1, 3, 2), (2, 1, 4, 2)]));
        assert_eq!(vec![9, 0], solve(2, vec![(1, 1, 3, 4), (3, 4, 6, 5)]));
    }
}
