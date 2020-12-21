/*
Problem 494. Target Sum
=======================

https://leetcode.com/problems/target-sum/

You are given a list of non-negative integers, a1, a2, ..., an, and a target, S. Now you have 2
symbols + and -. For each integer, you should choose one from + and - as its new symbol.

Find out how many ways to assign symbols to make sum of integers equal to target S.

Example 1:

    Input: nums is [1, 1, 1, 1, 1], S is 3.
    Output: 5
    Explanation:

    -1+1+1+1+1 = 3
    +1-1+1+1+1 = 3
    +1+1-1+1+1 = 3
    +1+1+1-1+1 = 3
    +1+1+1+1-1 = 3

    There are 5 ways to assign symbols to make the sum of nums be target 3.

Constraints:

    - The length of the given array is positive and will not exceed 20.
    - The sum of elements in the given array will not exceed 1000.
    - Your output answer is guaranteed to be fitted in a 32-bit integer.
*/

use std::collections::HashMap;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        let mut count = HashMap::new();
        count.insert(0, 1);
        let mut last_count = HashMap::new();
        for n in nums {
            std::mem::swap(&mut count, &mut last_count);
            count.clear();
            for (&k, &c) in last_count.iter() {
                *count.entry(k + n).or_insert(0) += c;
                *count.entry(k - n).or_insert(0) += c;
            }
        }
        count.get(&s).cloned().unwrap_or(0)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::find_target_sum_ways(Vec::new(), 9), 0);
        assert_eq!(Solution::find_target_sum_ways(vec![1], 0), 0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3));
    }
}
