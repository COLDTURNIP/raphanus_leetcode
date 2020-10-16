/*
Problem 219. Contains Duplicate II
==================================

https://leetcode.com/problems/contains-duplicate-ii/

Given an array of integers and an integer k, find out whether there are two distinct indices i and
j in the array such that nums[i] = nums[j] and the absolute difference between i and j is at most
k.

Example 1:

    Input: nums = [1,2,3,1], k = 3
    Output: true

Example 2:

    Input: nums = [1,0,1,1], k = 1
    Output: true

Example 3:

    Input: nums = [1,2,3,1,2,3], k = 2
    Output: false
*/

use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        if k <= 0 {
            return false;
        }
        let k = k as usize;
        // Use map here instead of maintain a sliding window as a set because the k is small enough
        // in Leetcode's test cases. This is faster, but is not memory efficient.
        let mut seen = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            if let Some(j) = seen.insert(n, i) {
                if i - j <= k {
                    return true;
                }
            }
        }
        false
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_window_3_found() {
        assert!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3));
    }

    #[test]
    fn test_window_1_found() {
        assert!(Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1));
    }

    #[test]
    fn test_window_2_not_found() {
        assert!(!Solution::contains_nearby_duplicate(
            vec![1, 2, 3, 1, 2, 3],
            2
        ));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::contains_nearby_duplicate(vec![0, 1, 0, 3, 12, 3, 1, 2, 3], 3));
    }
}
