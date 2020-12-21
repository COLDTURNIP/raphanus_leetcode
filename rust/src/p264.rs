/*
Problem 264. Ugly Number II
===========================

https://leetcode.com/problems/ugly-number-ii/

Write a program to find the n-th ugly number.

Ugly numbers are positive numbers whose prime factors only include 2, 3, 5.

Example:

    Input: n = 10
    Output: 12
    Explanation: 1, 2, 3, 4, 5, 6, 8, 9, 10, 12 is the sequence of the first 10 ugly numbers.

Note:

    - 1 is typically treated as an ugly number.
    - n does not exceed 1690.
*/

use std::collections::BTreeSet;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        if n < 1 {
            return -1;
        }
        let mut seen = BTreeSet::new();
        seen.insert(1_i32);
        for _i in 1..n {
            let min_entry = *seen.iter().next().unwrap();
            seen.remove(&min_entry);
            seen.insert(min_entry.checked_mul(2).unwrap_or(std::i32::MAX));
            seen.insert(min_entry.checked_mul(3).unwrap_or(std::i32::MAX));
            seen.insert(min_entry.checked_mul(5).unwrap_or(std::i32::MAX));
        }
        seen.into_iter().next().unwrap_or(-1)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_10() {
        assert_eq!(Solution::nth_ugly_number(10), 12);
    }

    #[test]
    fn test_invalid() {
        assert_eq!(Solution::nth_ugly_number(0), -1);
        assert_eq!(Solution::nth_ugly_number(-10), -1);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::nth_ugly_number(1690));
    }
}
