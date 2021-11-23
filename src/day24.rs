// https://leetcode.com/problems/min-stack
// https://leetcode.com/problems/min-stack/discuss/350778/My-Rust-solution-using-one-stack

use std::collections::VecDeque;

struct MinStack {
    min: i32,
    stack: VecDeque<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            min: std::i32::MAX,
            stack: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        if x <= self.min {
            self.stack.push_front(self.min);
            self.min = x;
        }
        self.stack.push_front(x);
    }

    fn pop(&mut self) {
        if !self.stack.is_empty() {
            if self.min == self.stack.pop_front().unwrap() {
                self.min = self.stack.pop_front().unwrap();
            }
        }
    }

    fn top(&self) -> i32 {
        *self.stack.front().unwrap()
    }

    fn get_min(&self) -> i32 {
        self.min
    }
}

#[cfg(test)]
mod tests {
    use super::MinStack;

    #[test]
    fn test_min_stack() {
        let mut stack = MinStack::new();
        stack.push(-2);
        stack.push(0);
        stack.push(-3);

        assert_eq!(-3, stack.get_min());

        stack.pop();

        assert_eq!(0, stack.top());
        assert_eq!(-2, stack.get_min());
    }
}
