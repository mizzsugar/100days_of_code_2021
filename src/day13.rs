// https://leetcode.com/problems/plus-one
// https://leetcode.com/problems/plus-one/discuss/282362/rust-simple-solution-0-ms-23-mb

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    for index in (0..digits.len()).rev() {
        if digits[index] == 9 {
            digits[index] = 0;
        } else {
            digits[index] += 1;
            return digits;
        }
    }
    digits.insert(0, 1);
    digits
}

#[cfg(test)]
mod tests {
    use super::plus_one;
    #[test]
    fn test_plus_one() {
        let nums = vec![4, 9, 5];
        let expected = vec![4, 9, 6];
        let actual = plus_one(nums);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_increment() {
        let nums = vec![9, 9];
        let expected = vec![1, 0, 0];
        let actual = plus_one(nums);
        assert_eq!(expected, actual);
    }
}
