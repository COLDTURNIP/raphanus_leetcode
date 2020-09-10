/*
Problem 53. Maximum Subarray
============================

https://leetcode.com/problems/maximum-subarray/

Given an integer array nums, find the contiguous subarray (containing at least one number) which
has the largest sum and return its sum.

Follow up: If you have figured out the O(n) solution, try coding another solution using the divide
and conquer approach, which is more subtle.

Example 1:

    Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
    Output: 6
    Explanation: [4,-1,2,1] has the largest sum = 6.

Example 2:

    Input: nums = [1]
    Output: 1

Example 3:

    Input: nums = [0]
    Output: 0

Example 4:

    Input: nums = [-1]
    Output: -1

Example 5:

    Input: nums = [-2147483647]
    Output: -2147483647

Constraints:

    1 <= nums.length <= 2 * 104
    -231 <= nums[i] <= 231 - 1
*/

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (_, _, ans, _) = Self::max_sub_slice(nums.as_slice());
        ans
    }

    fn max_sub_slice(nums: &[i32]) -> (i32, i32, i32, i32) {
        // Returns sum, lhs, middle, rhs:
        // - sum: sum of given numbers
        // - lhs: the maximum sum that the left most element is picked
        // - middle: the maximum continuous sum in given numbers
        // - rhs: the maximum sum that the right most element is picked
        let len = nums.len();
        match len {
            0 => (0, 0, 0, 0),
            1 => (nums[0], nums[0], nums[0], nums[0]),
            2 => {
                let (lhs, rhs, sum) = (nums[0], nums[1], nums[0] + nums[1]);
                (sum, sum.max(lhs), sum.max(lhs).max(rhs), sum.max(rhs))
            }
            _ => {
                let mid = len / 2;
                let (ls, ll, lm, lr) = Self::max_sub_slice(&nums[0..mid]);
                let (rs, rl, rm, rr) = Self::max_sub_slice(&nums[mid..len]);
                let mmax = (lr + rl).max(lm).max(rm);
                let lmax = (ls + rl).max(ll);
                let rmax = (rs + lr).max(rr);
                (ls + rs, lmax, mmax, rmax)
            }
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    #[test]
    fn test_normal() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }

    #[test]
    fn test_short() {
        assert_eq!(Solution::max_sub_array(vec![-2, -1]), -1);
    }

    #[test]
    fn test_variant() {
        assert_eq!(Solution::max_sub_array(vec![8, -19, 5, -4, 20]), 21);
    }

    #[test]
    fn test_variant2() {
        assert_eq!(
            Solution::max_sub_array(vec![1, 2, -1, -2, 2, 1, -2, 1, 4, -5, 4]),
            6
        );
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
        assert_eq!(Solution::max_sub_array(vec![0]), 0);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::max_sub_array(Vec::new()), 0);
    }

    #[test]
    fn test_i32_min() {
        assert_eq!(Solution::max_sub_array(vec![-2147483647]), -2147483647);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]));
    }
}
