// https://leetcode.com/problems/intersection-of-two-arrays-ii/
// https://leetcode.com/problems/intersection-of-two-arrays-ii/discuss/538633/Rust-solution-0ms

use std::collections::HashMap;

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for i in nums1 {
        let ptr = map.entry(i).or_insert(0);
        *ptr += 1;
    }
    let mut v = vec![];

    for i in nums2 {
        if let Some(t) = map.get_mut(&i) {
            if *t > 0 {
                v.push(i);
                *t -= 1;
            } else {
                map.remove_entry(&i);
            }
        }
    }

    v
}

#[cfg(test)]
mod tests {
    use super::intersect;
    #[test]
    fn test_intersect() {
        let num1 = vec![4, 9, 5];
        let num2 = vec![9, 4, 9, 8, 4];
        let expected = vec![4, 9];
        let mut actual = intersect(num1, num2);
        actual.sort();
        assert_eq!(expected, actual);
    }
}
