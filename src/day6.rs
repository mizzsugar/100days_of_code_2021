// https://leetcode.com/problems/remove-duplicates-from-sorted-array

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup
    nums.dedup();
    nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::remove_duplicates;
    #[test]
    fn test_remove_duplicates() {
        let actual = remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]);
        let expected: i32 = 5;
        assert_eq!(actual, expected);
    }
}
