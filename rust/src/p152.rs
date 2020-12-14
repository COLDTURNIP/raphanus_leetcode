/*
Problem 152. Maximum Product Subarray
=====================================

https://leetcode.com/problems/maximum-product-subarray/

Given an integer array nums, find the contiguous subarray within an array (containing at least one
number) which has the largest product.

Example 1:

    Input: [2,3,-2,4]
    Output: 6
    Explanation: [2,3] has the largest product 6.

Example 2:

    Input: [-2,0,-1]
    Output: 0
    Explanation: The result cannot be 2, because [-2,-1] is not a subarray.
*/

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        assert!(!nums.is_empty());
        // for each iteration i:
        // - cur_max: the maximum product of subarrays end at entry i
        // - cur_min: the minimum product of subarrays end at entry i
        let (mut cur_max, mut cur_min, mut ans) = (nums[0], nums[0], nums[0]);
        for n in nums.into_iter().skip(1) {
            let (max_n, min_n) = (cur_max * n, cur_min * n);
            cur_max = n.max(max_n).max(min_n);
            cur_min = n.min(max_n).min(min_n);
            ans = ans.max(cur_max);
        }
        ans
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::max_product(vec![2, 3, -2, 4, -7, 9, 9, 0, 1]),
            27216
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::max_product(vec![2, 3, -2, 4, -7, 9, 9, 0, 1]));
    }
}
