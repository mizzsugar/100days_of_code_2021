// https://leetcode.com/problems/two-sum
use std::collections::HashMap;

pub fn brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let length = nums.len();
    for i in 0..length {
        for j in i + 1..length {
            if nums[i] == target - nums[j] {
                return vec![i as i32, j as i32];
            }
        }
    }
    panic!("numbers unmatched")
}

pub fn one_pass_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hashmap: HashMap<&i32, i32> = HashMap::new();
    for i in 0..nums.len() {
        let complement = target - nums[i];
        if hashmap.contains_key(&complement) {
            return vec![i as i32, hashmap[&complement]];
        }
        *hashmap.entry(&nums[i]).or_insert(nums[i]) = i as i32;
    }
    panic!("numbers unmatched")
}

#[cfg(test)]
mod tests {
    use super::{brute_force, one_pass_hashmap};
    #[test]
    fn test_brute_force() {
        let mut actual = brute_force(vec![2, 7, 11, 15], 9);
        actual.sort();
        let expected = vec![0, 1];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_one_pass_hashmap() {
        let mut actual = one_pass_hashmap(vec![2, 7, 11, 15], 9);
        actual.sort();
        let expected = vec![0, 1];
        assert_eq!(actual, expected);
    }
}
