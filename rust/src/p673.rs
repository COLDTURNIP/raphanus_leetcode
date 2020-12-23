/*
Problem 673. Number of Longest Increasing Subsequence
=====================================================

https://leetcode.com/problems/number-of-longest-increasing-subsequence/

Given an integer array nums, return the number of longest increasing subsequences.

Notice that the sequence has to be strictly increasing.

Example 1:

    Input: nums = [1,3,5,4,7]
    Output: 2
    Explanation: The two longest increasing subsequences are [1, 3, 4, 7] and [1, 3, 5, 7].

Example 2:

    Input: nums = [2,2,2,2,2]
    Output: 5
    Explanation: The length of longest continuous increasing subsequence is 1, and there are 5
                 subsequences' length is 1, so output 5.

Constraints:

    1 <= nums.length <= 2000
    -10^6 <= nums[i] <= 10^6
*/

use std::cmp::Ordering::{Equal, Greater};

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let mut len_cnt: Vec<(i32, i32)> = Vec::with_capacity(nums.len());
        let mut max_len = 0;
        let mut max_len_cnt = 0;
        for (i, &n) in nums.iter().enumerate() {
            let mut prev_len = 0;
            let mut cur_cnt = 1;
            for j in 0..i {
                if n <= nums[j] {
                    continue;
                }
                let (len, cnt) = len_cnt[j];
                match len.cmp(&prev_len) {
                    Greater => {
                        prev_len = len;
                        cur_cnt = cnt;
                    }
                    Equal => cur_cnt += cnt,
                    _ => {}
                }
            }
            let cur_len = prev_len + 1;
            len_cnt.push((cur_len, cur_cnt));
            match cur_len.cmp(&max_len) {
                Greater => {
                    max_len = cur_len;
                    max_len_cnt = cur_cnt;
                }
                Equal => max_len_cnt += cur_cnt,
                _ => {}
            };
        }
        max_len_cnt
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(Solution::find_number_of_lis(vec![1, 3, 5, 4, 7]), 2);
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::find_number_of_lis(vec![2; 5]), 5);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::find_number_of_lis(vec![1, 3, 5, 4, 7]));
    }
}
