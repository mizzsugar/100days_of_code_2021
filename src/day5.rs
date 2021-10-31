// https://leetcode.com/problems/remove-element

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut i = 0;
    for j in 0..nums.len() {
        if nums[j] != val {
            nums[i] = nums[j];
            i += 1;
        }
    }
    i as i32
}

#[cfg(test)]
mod tests {
    use super::remove_element;
    #[test]
    fn test_remove_element() {
        let actual = remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2);
        let expected = 5;
        assert_eq!(actual, expected);
    }
}
