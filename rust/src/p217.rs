/*
Problem 217. Contains Duplicate
===============================

https://leetcode.com/problems/contains-duplicate/

Given an array of integers, find if the array contains any duplicates.

Your function should return true if any value appears at least twice in the array, and it should
return false if every element is distinct.

Example 1:

    Input: [1,2,3,1]
    Output: true

Example 2:

    Input: [1,2,3,4]
    Output: false

Example 3:

    Input: [1,1,1,3,3,4,3,2,4,2]
    Output: true
*/

use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::new();
        nums.into_iter().any(|n| !seen.insert(n))
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_unique() {
        assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_dupliated() {
        assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
    }

    #[test]
    fn test_dupliated_triple() {
        assert!(Solution::contains_duplicate(vec![
            1, 1, 1, 3, 3, 4, 3, 2, 4, 2
        ]));
    }

    #[test]
    fn test_empty() {
        assert!(!Solution::contains_duplicate(Vec::new()));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
    }
}
