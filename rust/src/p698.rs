/*
Problem 698. Partition to K Equal Sum Subsets
=============================================

https://leetcode.com/problems/partition-to-k-equal-sum-subsets/

Given an array of integers nums and a positive integer k, find whether it's possible to divide this
array into k non-empty subsets whose sums are all equal.

Example 1:

    Input: nums = [4, 3, 2, 3, 5, 2, 1], k = 4
    Output: True
    Explanation: It's possible to divide it into 4 subsets (5), (1, 4), (2,3), (2,3) with equal sums.

Note:

    - 1 <= k <= len(nums) <= 16.
    - 0 < nums[i] < 10000.
*/

impl Solution {
    fn can_partition_into(nums: &[i32], target: i32, state: &mut [i32]) -> bool {
        if nums.is_empty() {
            return true;
        }
        let n = nums[0];
        let rest = &nums[1..];
        let mut last_s = -1;
        for i in 0..state.len() {
            if state[i] == last_s {
                continue;
            }
            last_s = state[i];
            if state[i] + n <= target {
                state[i] += n;
                if Self::can_partition_into(rest, target, state) {
                    return true;
                }
                state[i] -= n;
            }
        }
        false
    }

    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % k != 0 {
            return false;
        }
        let target = sum / k;
        Self::can_partition_into(&nums, target, &mut vec![0; k as usize])
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_possible() {
        assert!(Solution::can_partition_k_subsets(
            vec![4, 3, 2, 3, 5, 2, 1],
            4
        ));
    }

    #[test]
    fn test_impossible() {
        assert!(!Solution::can_partition_k_subsets(vec![5, 2, 3], 4));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 1], 4));
    }
}
