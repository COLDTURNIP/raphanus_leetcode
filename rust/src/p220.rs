/*
Problem 220. Contains Duplicate III
=======================================

https://leetcode.com/problems/contains-duplicate-iii/

Given an array of integers, find out whether there are two distinct indices i and j in the array
such that the absolute difference between nums[i] and nums[j] is at most t and the absolute
difference between i and j is at most k.

I.e. given window size k and difference t, if there exists any i & j which
    - abs(i-j) <= k, and
    - abs(nums[i]-nums[j]) <= t

Example 1:

Input: nums = [1,2,3,1], k = 3, t = 0
Output: true
Example 2:

Input: nums = [1,0,1,1], k = 1, t = 2
Output: true
Example 3:

Input: nums = [1,5,9,1,5,9], k = 2, t = 3
Output: false

*/
use std::collections::{BTreeSet, VecDeque};
use std::ops::Bound::Included;

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
struct Point(i32, usize);

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        if k < 1 || t < 0 {
            return false;
        }
        let k = k as usize;
        let mut window = VecDeque::<Point>::with_capacity(k);
        let mut num_set = BTreeSet::<Point>::new();
        for (i, n) in nums.into_iter().enumerate() {
            if num_set
                .range((
                    Included(Point(n.saturating_sub(t), std::usize::MIN)),
                    Included(Point(n.saturating_add(t), std::usize::MAX)),
                ))
                .next()
                .is_some()
            {
                // any entry in the current number set falls the range n+-t
                return true;
            }
            if window.len() == k {
                let removed = window.pop_front().unwrap();
                num_set.remove(&removed);
            }
            window.push_back(Point(n, i));
            num_set.insert(Point(n, i));
        }
        false
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![1, 2, 3, 1], 3, 0),
            true
        );
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![1, 0, 1, 1], 1, 2),
            true
        );
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3),
            false
        );
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![0], 0, 0),
            false
        );
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![4, 1, 6, 3], 100, 1),
            true
        );
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![-1, 2_147_483_647], 1, 2_147_483_647),
            false
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3))
    }
}
