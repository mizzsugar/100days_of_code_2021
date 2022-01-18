// https://atcoder.jp/contests/typical90/tasks/typical90_bc

fn solve(n: usize, p: u64, q: u64, a: Vec<u64>) -> u64 {
    let mut ans = 0;
    for a in 0..n {
        for b in a + 1..n {
            let ab = (an[a] * an[b]) % p;
            for c in b + 1..n {
                let abc = (ab * an[c]) % p;
                for d in c + 1..n {
                    let abcd = (abc * an[d]) % p;
                    for e in d + 1..n {
                        if (abcd * an[e]) % p == q {
                            ans += 1;
                        }
                    }
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
        assert_eq!(252, solve(10, 1, 0, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
        assert_eq!(1, solve(6, 7, 1, vec![1, 2, 3, 4, 5, 6]));
    }
}