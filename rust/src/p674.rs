/*
Problem 674. Longest Continuous Increasing Subsequence
======================================================

https://leetcode.com/problems/longest-continuous-increasing-subsequence/

Given an unsorted array of integers nums, return the length of the longest continuous increasing
subsequence (i.e. subarray). The subsequence must be strictly increasing.

A continuous increasing subsequence is defined by two indices l and r (l < r) such that it is
[nums[l], nums[l + 1], ..., nums[r - 1], nums[r]] and for each l <= i < r, nums[i] < nums[i + 1].

Example 1:

    Input: nums = [1,3,5,4,7]
    Output: 3
    Explanation: The longest continuous increasing subsequence is [1,3,5] with length 3.
                 Even though [1,3,5,7] is an increasing subsequence, it is not continuous as
                 elements 5 and 7 are separated by element 4.

Example 2:

    Input: nums = [2,2,2,2,2]
    Output: 1
    Explanation: The longest continuous increasing subsequence is [2] with length 1. Note that it
                 must be strictly increasing.

Constraints:

    - 0 <= nums.length <= 10^4
    - -10^9 <= nums[i] <= 10^9
*/

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        if let Some(first) = nums.get(0).cloned() {
            nums.into_iter()
                .skip(1)
                .scan((first, 1), |state, n| {
                    if state.0 < n {
                        state.1 += 1;
                    } else {
                        state.1 = 1;
                    }
                    state.0 = n;
                    Some(state.1)
                })
                .max()
                .unwrap_or(1)
        } else {
            0
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_13547() {
        assert_eq!(Solution::find_length_of_lcis(vec![1, 3, 5, 4, 7]), 3);
    }

    #[test]
    fn test_all_2() {
        assert_eq!(Solution::find_length_of_lcis(vec![2, 2, 2, 2, 2]), 1);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::find_length_of_lcis(Vec::new()), 0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::find_length_of_lcis(vec![1, 3, 5, 4, 7]));
    }
}
