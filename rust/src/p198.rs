/*
Problem 198. House Robber
=========================

https://leetcode.com/problems/house-robber/

You are a professional robber planning to rob houses along a street. Each house has a certain
amount of money stashed, the only constraint stopping you from robbing each of them is that
adjacent houses have security system connected and it will automatically contact the police if two
adjacent houses were broken into on the same night.

Given a list of non-negative integers representing the amount of money of each house, determine the
maximum amount of money you can rob tonight without alerting the police.

Example 1:

    Input: nums = [1,2,3,1]
    Output: 4
    Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
                 Total amount you can rob = 1 + 3 = 4.

Example 2:

    Input: nums = [2,7,9,3,1]
    Output: 12
    Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
                 Total amount you can rob = 2 + 9 + 1 = 12.


Constraints:

    - 0 <= nums.length <= 100
    - 0 <= nums[i] <= 400
*/

use std::collections::HashMap;

impl Solution {
    fn rob_partial(
        nums: &[i32],
        start: usize,
        end: usize,
        cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if let Some(&ans) = cache.get(&(start, end)) {
            return ans;
        }
        let ans = match end - start {
            0 => 0,
            1 => nums[start],
            2 => nums[start].max(nums[start + 1]),
            _ => {
                let mut max = Solution::rob_partial(nums, start + 1, end, cache)
                    .max(Solution::rob_partial(nums, start, end - 1, cache));
                for i in start + 1..end - 1 {
                    let lhs = Solution::rob_partial(nums, start, i, cache);
                    let rhs = Solution::rob_partial(nums, i + 1, end, cache);
                    max = max.max(lhs + rhs);
                }
                max
            }
        };
        cache.insert((start, end), ans);
        ans
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut cache = HashMap::new();
        Solution::rob_partial(&nums, 0, nums.len(), &mut cache)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_1231() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn test_27931() {
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::rob(vec![2, 7, 9, 3, 1]));
    }
}
