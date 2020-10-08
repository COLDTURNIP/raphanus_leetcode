/*
Problem 155. Min Stack
======================

https://leetcode.com/problems/min-stack/

Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.

    push(x) -- Push element x onto stack.
    pop() -- Removes the element on top of the stack.
    top() -- Get the top element.
    getMin() -- Retrieve the minimum element in the stack.

Example 1:

    Input
    ["MinStack","push","push","push","getMin","pop","top","getMin"]
    [[],[-2],[0],[-3],[],[],[],[]]

    Output
    [null,null,null,null,-3,null,0,-2]

    Explanation
    MinStack minStack = new MinStack();
    minStack.push(-2);
    minStack.push(0);
    minStack.push(-3);
    minStack.getMin(); // return -3
    minStack.pop();
    minStack.top();    // return 0
    minStack.getMin(); // return -2


Constraints:

    Methods pop, top and getMin operations will always be called on non-empty stacks.
*/

#[derive(Default)]
struct MinStack {
    data: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            ..Default::default()
        }
    }

    fn push(&mut self, x: i32) {
        let new_min = self
            .data
            .last()
            .map_or(x, |(_, last_min)| *last_min.min(&x));
        self.data.push((x, new_min));
    }

    fn pop(&mut self) {
        self.data.pop();
    }

    fn top(&self) -> i32 {
        self.data.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.data.last().unwrap().1
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::MinStack;

    #[test]
    fn test() {
        let mut obj = MinStack::new();
        obj.push(-2);
        obj.push(0);
        obj.push(-3);
        assert_eq!(obj.get_min(), -3);
        obj.pop();
        assert_eq!(obj.top(), 0);
        assert_eq!(obj.get_min(), -2);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            let mut obj = MinStack::new();
            obj.push(-2);
            obj.push(0);
            obj.push(-3);
            obj.get_min();
            obj.pop();
            obj.top();
            obj.get_min();
        });
    }
}
