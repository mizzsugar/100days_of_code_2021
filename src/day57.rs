// https://atcoder.jp/contests/typical90/tasks/typical90_al
use num::integer::lcm;

fn solve(a: u128, b: u128) -> String {
    let ans = lcm(a, b);
    if ans > 10_u128.pow(18) {
        return String::from("Large")
    }
    String::from("{}", ans)
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(String::from("12"), solve(4, 6));
        assert_eq!(String::from("Large"), solve(1000000000000000000, 3));
    }
}
