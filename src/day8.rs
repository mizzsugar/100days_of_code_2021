// https://leetcode.com/problems/contains-duplicate/
use std::collections::HashSet;

fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut s: HashSet<i32> = HashSet::new();
    for num in nums {
        if s.contains(&num) {
            return true;
        }
        s.insert(num);
    }
    false
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
    use super::contains_duplicate;
    #[test]
    fn test_contains_duplicate() {
        assert!(!contains_duplicate(vec![1, 2, 3, 4, 5, 6, 7]));
        assert!(contains_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 7]));
    }
}
