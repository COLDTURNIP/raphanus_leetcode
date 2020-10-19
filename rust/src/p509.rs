/*
Problem 509. Fibonacci Number
=============================

https://leetcode.com/problems/fibonacci-number/

The Fibonacci numbers, commonly denoted F(n) form a sequence, called the Fibonacci sequence, such
that each number is the sum of the two preceding ones, starting from 0 and 1. That is,

    F(0) = 0,   F(1) = 1
    F(N) = F(N - 1) + F(N - 2), for N > 1.
    Given N, calculate F(N).

Example 1:

    Input: 2
    Output: 1
    Explanation: F(2) = F(1) + F(0) = 1 + 0 = 1.

Example 2:

    Input: 3
    Output: 2
    Explanation: F(3) = F(2) + F(1) = 1 + 1 = 2.

Example 3:

    Input: 4
    Output: 3
    Explanation: F(4) = F(3) + F(2) = 2 + 1 = 3.

Note:

    0 ≤ N ≤ 30.
*/

use std::collections::HashMap;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut cache = HashMap::new();
        cache.insert(1, 1);
        Self::fib_impl(n, &mut cache)
    }

    fn fib_impl(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
        if n <= 0 {
            0
        } else if let Some(&ans) = cache.get(&n) {
            ans
        } else {
            let ans = Self::fib_impl(n - 2, cache) + Self::fib_impl(n - 1, cache);
            cache.insert(n, ans);
            ans
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_2() {
        assert_eq!(Solution::fib(2), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::fib(3), 2);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::fib(4), 3);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::fib(30));
    }
}
