// https://leetcode.com/problems/merge-sorted-array
// https://leetcode.com/problems/merge-sorted-array/discuss/1174582/Rust-0ms-1.9Mb

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    nums1.truncate(m as usize);
    nums1.append(nums2);
    nums1.sort();
}

#[cfg(test)]
mod tests {
    use super::merge;

    #[test]
    fn test_merge() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;

        let expected = vec![1, 2, 2, 3, 5, 6];

        merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(expected, nums1)
    }
}
