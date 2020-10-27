/*
Problem 697. Degree of an Array
===============================

https://leetcode.com/problems/degree-of-an-array/

Given a non-empty array of non-negative integers nums, the degree of this array is defined as the
maximum frequency of any one of its elements.

Your task is to find the smallest possible length of a (contiguous) subarray of nums, that has the
same degree as nums.

Example 1:

    Input: nums = [1,2,2,3,1]
    Output: 2
    Explanation:
        The input array has a degree of 2 because both elements 1 and 2 appear twice.
        Of the subarrays that have the same degree:
        [1, 2, 2, 3, 1], [1, 2, 2, 3], [2, 2, 3, 1], [1, 2, 2], [2, 2, 3], [2, 2]
        The shortest length is 2. So return 2.

Example 2:

    Input: nums = [1,2,2,3,1,4,2]
    Output: 6
    Explanation:
        The degree is 3 because the element 2 is repeated 3 times.
        So [2,2,3,1,4,2] is the shortest subarray, therefore returning 6.

Constraints:

    - nums.length will be between 1 and 50,000.
    - nums[i] will be an integer between 0 and 49,999.
*/

use std::cmp::Ordering::{Equal, Greater};
use std::collections::HashMap;

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut cnt = HashMap::new();
        for (i, n) in nums.into_iter().enumerate() {
            let entry = cnt.entry(n).or_insert((0, i, i));
            entry.0 += 1;
            entry.2 = i;
        }
        let mut max_c = 0;
        let mut min_len = 0;
        for (_, (c, start, end)) in cnt.into_iter() {
            match c.cmp(&max_c) {
                Greater => {
                    max_c = c;
                    min_len = end - start;
                }
                Equal => min_len = min_len.min(end - start),
                _ => {}
            }
        }
        min_len as i32 + 1
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_12231() {
        assert_eq!(Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1]), 2);
    }

    #[test]
    fn test_1223142() {
        assert_eq!(
            Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1, 4, 2]),
            6
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1, 4, 2]));
    }
}
