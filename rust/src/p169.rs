/*
Problem 169. Majority Element
=============================

https://leetcode.com/problems/majority-element/

Given an array of size n, find the majority element. The majority element is the element that
appears more than ⌊ n/2 ⌋ times.

You may assume that the array is non-empty and the majority element always exist in the array.

Example 1:

    Input: [3,2,3]
    Output: 3

Example 2:

    Input: [2,2,1,1,1,2,2]
    Output: 2
*/

//use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((0, 0), |(major, cnt), n| {
                if major == n {
                    (major, cnt + 1)
                } else if cnt <= 1 {
                    (n, 1)
                } else {
                    (major, cnt - 1)
                }
            })
            .0
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_323() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    }

    #[test]
    fn test_2211122() {
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::majority_element(Vec::new()), 0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::majority_element(vec![1, 1, 4, 4, 3, 4, 4, 4, 4, 5]));
    }
}
