// https://leetcode.com/problems/https://leetcode.com/problems/single-number/

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut result: i32 = 0;
    for n in nums {
        result ^= n;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::single_number;
    #[test]
    fn test_single_number() {
        let expected: i32 = 4;
        let actual = single_number(vec![4, 1, 2, 1, 2]);
        assert_eq!(actual, expected);
    }
}
