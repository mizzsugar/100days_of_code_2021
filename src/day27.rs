// https://leetcode.com/problems/count-primes
// https://leetcode.com/problems/count-primes/discuss/549492/Rust-4ms-solution

pub fn count_primes(n: i32) -> i32 {
    let mut not_prime: Vec<bool> = vec![false; n as usize];
    let mut count = 0;
    for i in 2..n as usize {
        if !not_prime[i] {
            count += 1;
        }
        let mut j = 2;
        while j * i < n as usize {
            not_prime[j * i] = true;
            j += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::count_primes;

    #[test]
    fn test_count_primes() {
        let expected = 4;
        let actual = count_primes(10);
        assert_eq!(expected, actual)
    }
}
