// https://github.com/E869120/kyopro_educational_90/blob/main/problem-txt/001.txt

fn solve(n: u32, l: u32, k: u32, a: &Vec<u32>, mid: &u32) -> bool {
    let mut cnt: u32 = 0;
    let mut pre: u32 = 0;
    for i in 0..n as usize {
        if a[i] - pre >= *mid && l - a[i] >= *mid {
            cnt += 1;
            pre = a[i];
        }
    }
    cnt >= k
}

fn main(n: u32, l: u32, k: u32, a: Vec<u32>) -> u32 {
    let mut left: u32 = 0;
    let mut right: u32 = l;
    while right - left > 1 {
        let mid = left + (right - left) / 2;
        if solve(n, l, k, &a, &mid) {
            left = mid;
        } else {
            right = mid;
        }
    }
    left
}

#[cfg(test)]
mod tests {
    use super::main;
    #[test]
    fn test_solve() {
        assert_eq!(12, main(7, 45, 2, vec![7, 11, 16, 20, 28, 34, 38]));
        assert_eq!(46, main(3, 100, 1, vec![28, 54, 81]));
        assert_eq!(26, main(3, 100, 2, vec![28, 54, 81]));
    }
}
