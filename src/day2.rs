// https://leetcode.com/problems/majority-element

pub fn brute_force(nums: Vec<i32>) -> i32 {
    let majority_count_float = (nums.len() / 2) as f64;
    let majority_count = majority_count_float.floor() as i32;
    for num in &nums {
        let mut count = 0;
        for elm in &nums {
            if elm == num {
                count += 1;
            }
        }
        if count > majority_count {
            return *num;
        }
    }
    panic!("cannot find majority element")
}

pub fn calc_by_sorting(nums: Vec<i32>) -> i32 {
    let mut cloned = nums.clone();
    cloned.sort();
    let folding_point_float = (nums.len() / 2) as f64;
    let folding_point = folding_point_float.floor() as usize;
    return cloned[folding_point];
}

#[cfg(test)]
mod tests {
    use super::{brute_force, calc_by_sorting};
    #[test]
    fn test_brute_force() {
        let actual = brute_force(vec![2, 2, 1, 1, 1, 2, 2]);
        let expected = 2;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_calc_by_sorting() {
        let actual = calc_by_sorting(vec![2, 2, 1, 1, 1, 2, 2]);
        let expected = 2;
        assert_eq!(actual, expected);
    }
}
