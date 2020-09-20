/*
Problem 81. Search in Rotated Sorted Array II
=============================================

https://leetcode.com/problems/search-in-rotated-sorted-array-ii/

Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.

(i.e., [0,0,1,2,2,5,6] might become [2,5,6,0,0,1,2]).

You are given a target value to search. If found in the array return true, otherwise return false.

Example 1:

    Input: nums = [2,5,6,0,0,1,2], target = 0
    Output: true

Example 2:

    Input: nums = [2,5,6,0,0,1,2], target = 3
    Output: false

Follow up:

This is a follow up problem to Search in Rotated Sorted Array, where nums may contain duplicates.
Would this affect the run-time complexity? How and why?
*/

use std::cmp::Ordering::{Equal, Greater, Less};

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.is_empty() {
            return false;
        }
        const SHORT: usize = 5;

        let len = nums.len();
        let (mut lb, mut rb) = (0, len);
        while rb.saturating_sub(lb + SHORT) > 0 {
            let mid = (lb + rb) / 2;
            let (lv, mv, rv) = (nums[lb], nums[mid], nums[rb - 1]);
            let take = match (target.cmp(&lv), target.cmp(&mv), target.cmp(&rv)) {
                (Equal, _, _) | (_, Equal, _) | (_, _, Equal) => return true,
                (_, cmp, _) if lv < rv => cmp, // conventional binary search
                (Less, _, Greater) => return false,
                (Greater, cmp, Greater) => match mv.cmp(&lv) {
                    Equal => Equal, // undecidable case
                    Less => Less,
                    Greater => cmp,
                },
                (Less, cmp, Less) => match mv.cmp(&rv) {
                    Equal => Equal, // undecidable case
                    Greater => Greater,
                    Less => cmp,
                },
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
                // special case: cannot decide to take rhs or lhs
                Equal => lb += 1,
            }
        }

        // downgrade to linear search for short array
        (lb..rb).any(|i| nums[i] == target)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    #[test]
    fn test_normal_found() {
        assert_eq!(Solution::search(vec![1, 2, 0, 1, 1, 1], 0), true);
    }

    #[test]
    fn test_normal_dup_found_rhs() {
        assert_eq!(Solution::search(vec![1, 2, 2, 2, 0, 1, 1], 0), true);
    }

    #[test]
    fn test_normal_not_found() {
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
    }

    #[test]
    fn test_4to2_find_0_rhs() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), true);
    }

    #[test]
    fn test_4to2_find_0_lhs() {
        assert_eq!(Solution::search(vec![7, 0, 1, 2, 4, 5, 6], 0), true);
    }

    #[test]
    fn test_4to2_not_found() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), false);
    }

    #[test]
    fn test_single_element_list() {
        assert_eq!(Solution::search(vec![0], 0), true);
        assert_eq!(Solution::search(vec![3], 0), false);
    }

    #[test]
    fn test_empty_not_found() {
        assert_eq!(Solution::search(Vec::new(), 0), false);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::search(vec![4, 5, 6, 7, 8, 9, 10, 11, 0, 1, 2, 3], 0));
    }
}
