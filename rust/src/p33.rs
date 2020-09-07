/*
Problem 33. Search in Rotated Sorted Array
==========================================

https://leetcode.com/problems/search-in-rotated-sorted-array/

You are given an integer array nums sorted in ascending order, and an integer target.

Suppose that nums is rotated at some pivot unknown to you beforehand (i.e., [0,1,2,4,5,6,7] might
become [4,5,6,7,0,1,2]).

If target is found in the array return its index, otherwise, return -1.



Example 1:

    Input: nums = [4,5,6,7,0,1,2], target = 0
    Output: 4

Example 2:

    Input: nums = [4,5,6,7,0,1,2], target = 3
    Output: -1

Example 3:

    Input: nums = [1], target = 0
    Output: -1

Constraints:

    1 <= nums.length <= 5000
    -10^4 <= nums[i] <= 10^4
    All values of nums are unique.
    nums is guranteed to be rotated at some pivot.
    -10^4 <= target <= 10^4
*/
use std::cmp::Ordering::{Equal, Greater, Less};

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        const SHORT: usize = 5;

        let len = nums.len();
        let (mut lb, mut rb) = (0, len);
        while rb.saturating_sub(lb + SHORT) > 0 {
            let mid = (lb + rb) / 2;
            let (lv, mv, rv) = (nums[lb], nums[mid], nums[rb - 1]);
            let take = match (target.cmp(&lv), target.cmp(&mv), target.cmp(&rv)) {
                (Equal, _, _) => return lb as i32,
                (_, Equal, _) => return mid as i32,
                (_, _, Equal) => return rb as i32 - 1,
                (_, cmp, _) if lv < rv => cmp, // conventional binary search
                (Less, _, Greater) => return -1,
                (Greater, cmp, Greater) => {
                    if lv < mv {
                        cmp
                    } else {
                        Less
                    }
                }
                (Less, cmp, Less) => {
                    if mv < rv {
                        cmp
                    } else {
                        Greater
                    }
                }
                _ => unreachable!(
                    "{} compare to #{}({}), #{}({}), #{}({})",
                    target,
                    lb,
                    lv,
                    mid,
                    mv,
                    rb - 1,
                    rv
                ),
            };
            match take {
                Less => rb = mid,
                Greater => lb = mid + 1,
                _ => unreachable!("leaking equal situation"),
            }
        }

        // downgrade to linear search for short array
        (lb..rb)
            .find(|&i| nums[i] == target)
            .map(|i| i as i32)
            .unwrap_or(-1)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    #[test]
    fn test_normal_found() {
        assert_eq!(Solution::search(vec![1, 2, 3, 4, 6, 7, 8, 9], 4), 3);
    }

    #[test]
    fn test_normal_not_found() {
        assert_eq!(Solution::search(vec![1, 2, 3, 4, 6, 7, 8, 9], 5), -1);
    }

    #[test]
    fn test_4to2_find_0_rhs() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }

    #[test]
    fn test_4to2_find_0_lhs() {
        assert_eq!(Solution::search(vec![7, 0, 1, 2, 4, 5, 6], 0), 1);
    }

    #[test]
    fn test_4to2_not_found() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }

    #[test]
    fn test_single_element_list() {
        assert_eq!(Solution::search(vec![0], 0), 0);
        assert_eq!(Solution::search(vec![3], 0), -1);
    }

    #[test]
    fn test_empty_not_found() {
        assert_eq!(Solution::search(Vec::new(), 0), -1);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::search(vec![4, 5, 6, 7, 8, 9, 10, 11, 0, 1, 2, 3], 0));
    }
}
