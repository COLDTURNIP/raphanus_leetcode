/*
Problem 377. Combination Sum IV
===============================

https://leetcode.com/problems/combination-sum-iv/

Given an integer array with all positive numbers and no duplicates, find the number of possible
combinations that add up to a positive integer target.

Example:

    nums = [1, 2, 3]
    target = 4

    The possible combination ways are:
    (1, 1, 1, 1)
    (1, 1, 2)
    (1, 2, 1)
    (1, 3)
    (2, 1, 1)
    (2, 2)
    (3, 1)

    Note that different sequences are counted as different combinations.

    Therefore the output is 7.

Follow up:
    - What if negative numbers are allowed in the given array?
    - How does it change the problem?
    - What limitation we need to add to the question to allow negative numbers?

Credits:
    Special thanks to @pbrother for adding this problem and creating all test cases.
*/

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut cache = Vec::with_capacity(target as usize + 1);
        cache.push(1);
        for t in 1..=(target as usize) {
            let mut cnt = 0;
            for n in nums.iter() {
                cnt += t
                    .checked_sub(*n as usize)
                    .and_then(|t| cache.get(t))
                    .cloned()
                    .unwrap_or(0);
            }
            cache.push(cnt)
        }
        cache.get(target as usize).cloned().unwrap_or(0)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::combination_sum4(vec![5, 1, 8], 24), 982);
    }

    #[test]
    fn test_zero() {
        assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 0), 1);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::combination_sum4(Vec::new(), 4), 0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::combination_sum4(vec![1, 2, 3], 4));
    }
}
