// https://leetcode.com/problems/shuffle-an-array
// https://leetcode.com/problems/shuffle-an-array/discuss/737863/Rust-Solution

use rand::seq::SliceRandom;

struct Solution {
    original: Vec<i32>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self { original: nums }
    }

    fn reset(&self) -> Vec<i32> {
        self.original.clone()
    }

    fn shuffle(&self) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let mut array = self.original.clone();
        array.shuffle(&mut rng);
        array
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_new() {
        let actual = Solution::new(vec![1, 2, 3]);
        let expected = vec![1, 2, 3];
        assert_eq!(expected, actual.original);
    }

    #[test]
    fn test_suffle() {
        let nums = Solution::new(vec![1, 2, 3]);
        assert_ne!(nums.original, nums.shuffle());

        assert_eq!(nums.original, nums.reset());
    }
}
