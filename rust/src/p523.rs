/*
Problem 523. Continuous Subarray Sum
====================================

https://leetcode.com/problems/continuous-subarray-sum/

Given a list of non-negative numbers and a target integer k, write a function to check if the array
has a continuous subarray of size at least 2 that sums up to a multiple of k, that is, sums up to
n*k where n is also an integer.

Example 1:

    Input: [23, 2, 4, 6, 7],  k=6
    Output: True
    Explanation: Because [2, 4] is a continuous subarray of size 2 and sums up to 6.

Example 2:

    Input: [23, 2, 6, 4, 7],  k=6
    Output: True
    Explanation: Because [23, 2, 6, 4, 7] is an continuous subarray of size 5 and sums up to 42.

Constraints:

    The length of the array won't exceed 10,000.
    You may assume the sum of all the numbers is in the range of a signed 32-bit integer.
*/

use std::collections::HashMap;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let k = k.abs();
        let mut seen = HashMap::<i32, i32>::new();
        seen.insert(0, -1);
        let mut rem = 0;
        for (end, n) in nums.into_iter().enumerate() {
            let end = end as i32;
            let next_rem = rem + n;
            rem = next_rem.checked_rem(k).unwrap_or(next_rem);
            if let Some(&start) = seen.get(&rem) {
                if end - start >= 2 {
                    return true;
                }
            } else {
                seen.insert(rem, end as i32);
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
    fn test_possible() {
        assert!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6));
        assert!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 6));
        assert!(Solution::check_subarray_sum(vec![5, 0, 0], 0));
    }

    #[test]
    fn test_impossible() {
        assert!(!Solution::check_subarray_sum(vec![23, 5], 3));
        assert!(!Solution::check_subarray_sum(vec![0, 1], 2));
        assert!(!Solution::check_subarray_sum(Vec::new(), 3));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6));
    }
}
