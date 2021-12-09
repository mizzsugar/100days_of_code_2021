// https://atcoder.jp/contests/abc088/tasks/abc088_b

fn solve(mut nums: Vec<u32>) -> u32 {
    nums.sort_by(|x, y| x.cmp(y).reverse());
    let mut a = 0;
    let mut b = 0;
    for (i, &x) in nums.iter().enumerate() {
        if i % 2 == 0 {
            a += x;
        } else {
            b += x;
        }
    }
    a - b
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(2, solve(vec![2, 7, 4]));
        assert_eq!(4, solve(vec![20, 18, 2, 18]));
    }
}
