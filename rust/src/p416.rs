/*
Problem 416. Partition Equal Subset Sum
=======================================

https://leetcode.com/problems/partition-equal-subset-sum/

Given a non-empty array nums containing only positive integers, find if the array can be
partitioned into two subsets such that the sum of elements in both subsets is equal.

Example 1:

    Input: nums = [1,5,11,5]
    Output: true
    Explanation: The array can be partitioned as [1, 5, 5] and [11].

Example 2:

    Input: nums = [1,2,3,5]
    Output: false
    Explanation: The array cannot be partitioned into equal sum subsets.

Constraints:

    - 1 <= nums.length <= 200
    - 1 <= nums[i] <= 100
*/

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum & 1 == 1 {
            return false;
        } else if sum == 0 {
            return true;
        }
        let target = sum as usize / 2;
        let mut seen = vec![false; target as usize + 1];
        seen[0] = true;
        for n in nums.into_iter().map(|n| n as usize) {
            for s in (0..=target.saturating_sub(n)).rev() {
                let seen_s = seen[s];
                if let Some(entry) = seen.get_mut(s + n) {
                    *entry |= seen_s;
                }
            }
        }
        seen[target]
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_possible() {
        assert!(Solution::can_partition(vec![1, 5, 11, 5]));
        assert!(Solution::can_partition(vec![14, 9, 8, 4, 3, 2]));
    }

    #[test]
    fn test_impossible() {
        assert!(!Solution::can_partition(vec![1, 2, 5]));
        assert!(!Solution::can_partition(vec![1, 2, 3, 5]));
    }

    #[test]
    fn test_empty() {
        assert!(Solution::can_partition(Vec::new()));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::can_partition(vec![1, 5, 11, 5]));
    }
}
