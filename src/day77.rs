// https://atcoder.jp/contests/typical90/tasks/typical90_ah

use std::collections::{BTreeMap, VecDeque};


fn solve(n: usize, k: u32, a: Vec<u32>) -> usize {
    let mut line = VecDeque::<u32>::new();
    let mut map = BTreeMap::<u32, u32>::new();
    let mut max = 0;
    for a in a {
        if !map.contains_key(&a) {
            while map.len() == k as usize {
                let top = line.pop_front().unwrap();
                map.entry(top).and_modify(|e| *e -= 1);
                if map[&top] == 0 {
                    map.remove(&top);
                }
            }
        }
        line.push_back(a);
        *map.entry(a).or_default() += 1;
        max = max.max(line.len());
    }
    max
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(1, solve(5, 1, vec![1, 2, 3, 4, 5]));
        assert_eq!(5, solve(5, 1, vec![1, 2, 3, 4, 5]));
    }
}
