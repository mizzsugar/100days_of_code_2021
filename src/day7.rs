// https://leetcode.com/problems/rotate-array

fn rotate(nums: &mut Vec<i32>, k: i32) {
    let i = k as usize % nums.len();
    reverse(nums, 0, nums.len() - 1);
    reverse(nums, 0, i - 1);
    reverse(nums, i, nums.len() - 1);
}

fn reverse(nums: &mut Vec<i32>, start: usize, end: usize) {
    let mut start_ = start.clone();
    let mut end_ = end.clone();
    while start_ < end_ {
        let tmp = nums[start_];
        nums[start_] = nums[end_];
        nums[end_] = tmp;
        start_ += 1;
        end_ -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::rotate;
    #[test]
    fn test_rotate() {
        let mut v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
        let expected: Vec<i32> = vec![5, 6, 7, 1, 2, 3, 4];
        rotate(&mut v, 3);
        assert_eq!(v, expected);
    }
}
