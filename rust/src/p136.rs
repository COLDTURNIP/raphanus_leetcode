/*
Problem 136. Single Number
==========================

https://leetcode.com/problems/single-number/

Given a non-empty array of integers nums, every element appears twice except for one. Find that
single one.

Follow up: Could you implement a solution with a linear runtime complexity and without using extra
memory?

Example 1:

    Input: nums = [2,2,1]
    Output: 1

Example 2:

    Input: nums = [4,1,2,1,2]
    Output: 4

Example 3:

    Input: nums = [1]
    Output: 1

Constraints:

    - 1 <= nums.length <= 3 * 104
    - -3 * 104 <= nums[i] <= 3 * 104
    - Each element in the array appears twice except for one element which appears only once.
*/

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |ans, n| ans ^ n)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_221() {
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
    }

    #[test]
    fn test_41212() {
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::single_number(vec![1]), 1);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::single_number(vec![4, 1, 2, 1, 2]));
    }
}
