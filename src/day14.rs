// https://leetcode.com/problems/move-zeroes

pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut last_non_zero_found_at = 0;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums[last_non_zero_found_at] = nums[i];
            last_non_zero_found_at += 1;
        }
    }
    for i in last_non_zero_found_at..nums.len() {
        nums[i] = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::move_zeroes;
    #[test]
    fn test_plus_one() {
        let mut nums = vec![0, 1, 0, 3, 12];
        let expected = vec![1, 3, 12, 0, 0];
        move_zeroes(&mut nums);
        assert_eq!(expected, nums);
    }
}
