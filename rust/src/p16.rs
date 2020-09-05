/*
Problem 16. 3Sum Closest
========================

https://leetcode.com/problems/3sum-closest/

Given an array nums of n integers and an integer target, find three integers in nums such that the
sum is closest to target. Return the sum of the three integers. You may assume that each input
would have exactly one solution.

Example 1:

    Input: nums = [-1,2,1,-4], target = 1
    Output: 2
    Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).

Constraints:

    3 <= nums.length <= 10^3
    -10^3 <= nums[i] <= 10^3
    -10^4 <= target <= 10^4
*/

use std::cmp::Ordering::{Equal, Greater, Less};

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        fn closest(a: &i32, b: &i32, target: &i32) -> i32 {
            if (a - target).abs() < (b - target).abs() {
                *a
            } else {
                *b
            }
        };

        let mut nums = nums;
        nums.sort_unstable();
        let len = nums.len();
        let mut ans = None::<i32>;
        for i in 0..len {
            let picked = nums[i];
            let mut li = i + 1;
            let mut ri = len - 1;
            while li < ri {
                let sum = picked + nums[li] + nums[ri];
                match target.cmp(&sum) {
                    Less => {
                        ans = ans.map(|n| closest(&n, &sum, &target)).or(Some(sum));
                        ri -= 1;
                    }
                    Greater => {
                        ans = ans.map(|n| closest(&n, &sum, &target)).or(Some(sum));
                        li += 1;
                    }
                    Equal => return target,
                }
            }
        }
        ans.unwrap_or(target)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    #[test]
    fn test_short() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }

    #[test]
    fn test_ones() {
        assert_eq!(Solution::three_sum_closest(vec![1, 1, -1, -1, 3], -1), -1);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::three_sum_closest(vec![-1, 2, 1, -4], 1))
    }
}
