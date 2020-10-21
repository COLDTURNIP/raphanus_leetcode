/*
Problem 643. Maximum Average Subarray I
=======================================

https://leetcode.com/problems/maximum-average-subarray-i/

Given an array consisting of n integers, find the contiguous subarray of given length k that has
the maximum average value. And you need to output the maximum average value.

Example 1:

    Input: [1,12,-5,-6,50,3], k = 4
    Output: 12.75
    Explanation: Maximum average is (12-5-6+50)/4 = 51/4 = 12.75

Note:

    - 1 <= k <= n <= 30,000.
    - Elements of the given array will be in the range [-10,000, 10,000].
*/

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k.max(0) as usize;
        let len = nums.len();
        let mut sum: i32 = nums.iter().take(k).sum();
        let mut max_sum = sum;
        for i in k..len {
            sum = sum + nums[i] - nums[i - k];
            max_sum = max_sum.max(sum);
        }
        max_sum as f64 / k as f64
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    fn check(nums: Vec<i32>, k: i32, expect: f64) {
        assert!((Solution::find_max_average(nums, k) - expect).abs() < f64::EPSILON);
    }

    #[test]
    fn test_avg_4() {
        check(vec![1, 12, -5, -6, 50, 3], 4, 12.75);
    }

    #[test]
    fn test_empty() {
        check(Vec::new(), 10, 0.0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::find_max_average(vec![3, 1, 2, 10, 1], 3));
    }
}
