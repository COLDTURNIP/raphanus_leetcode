/*
Problem 18. 4Sum
=======================================

https://leetcode.com/problems/4sum/

Given array nums = [1, 0, -1, 0, -2, 2], and target = 0.

A solution set is:
[
  [-1,  0, 0, 1],
  [-2, -1, 1, 2],
  [-2,  0, 0, 2]
]

*/
use std::cmp::Ordering::{Equal, Greater, Less};

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut result = Self::n_sum(&nums, target, 4, 4);
        result.iter_mut().for_each(|v| v.reverse());
        result
    }

    fn two_sum(nums: &[i32], target: i32, cap: usize) -> Vec<Vec<i32>> {
        let mut ret = Vec::with_capacity(nums.len() / 2);
        if nums.len() < 2 {
            return ret;
        }
        let mut nums = Vec::from(nums);
        nums.sort_unstable();
        let mut liter = nums.iter().enumerate();
        let mut riter = nums.iter().enumerate().rev();
        let mut last_found = None;
        let mut left = liter.next();
        let mut right = riter.next();
        loop {
            match (left, right) {
                (Some((_, &lval)), Some((_, &rval))) if last_found == Some((lval, rval)) => {
                    left = liter.next();
                    right = riter.next();
                }
                (Some((i, &lval)), Some((j, &rval))) if i < j => match target.cmp(&(lval + rval)) {
                    Less => right = riter.next(),
                    Greater => left = liter.next(),
                    Equal => {
                        let mut result = Vec::with_capacity(cap);
                        result.push(rval);
                        result.push(lval);
                        ret.push(result);
                        last_found = Some((lval, rval));
                        left = liter.next();
                        right = riter.next();
                    }
                },
                _ => break,
            }
        }
        ret
    }

    fn n_sum(nums: &[i32], target: i32, n: usize, cap: usize) -> Vec<Vec<i32>> {
        assert!(n > 1, "invalid n={}", n);
        assert!(cap >= n, "invalid cap={} < n={}", cap, n);
        if n == 2 {
            Self::two_sum(nums, target, cap)
        } else {
            let mut ret = Vec::with_capacity(nums.len() * nums.len());
            let mut prev = None;
            for (i, &pick) in nums.iter().take(nums.len() - n + 1).enumerate() {
                if prev == Some(pick) {
                    continue;
                }
                prev = Some(pick);
                let mut comb = Self::n_sum(&nums[i + 1..], target - pick, n - 1, cap);
                comb.iter_mut().for_each(|v| v.push(pick));
                ret.extend(comb);
            }
            ret
        }
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
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );
        assert_eq!(
            Solution::four_sum(vec![1, -2, -5, -4, -3, 3, 3, 5], -11),
            vec![vec![-5, -4, -3, 1]]
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::four_sum(vec![1, -2, -5, -4, -3, 3, 3, 5], -11))
    }
}
