// https://leetcode.com/problems/first-bad-version
// https://leetcode.com/problems/first-bad-version/discuss/1522282/Rust-simple-solution

fn is_bad_version(versions: i32) -> bool {
    if versions >= 4 {
        return true;
    }
    false
}

fn first_bad_version(n: i32) -> i32 {
    let mut l: i32 = 1;
    let mut r: i32 = n;
    let mut res: i32 = 1;

    while l <= r {
        let mid = (((l as i128) + (r as i128)) / 2) as i32;
        if is_bad_version(mid) {
            res = mid;
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::first_bad_version;

    #[test]
    fn test_first_bad_version() {
        let expected = 4;
        let actual = first_bad_version(5);

        assert_eq!(expected, actual)
    }
}
