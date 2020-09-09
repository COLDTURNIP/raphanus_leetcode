/*
Problem 55. Jump Game
==================================

https://leetcode.com/problems/jump-game/

Given an array of non-negative integers, you are initially positioned at the first index of the
array.

Each element in the array represents your maximum jump length at that position.

Determine if you are able to reach the last index.

Example 1:

    Input: nums = [2,3,1,1,4]
    Output: true
    Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.

Example 2:

    Input: nums = [3,2,1,0,4]
    Output: false
    Explanation: You will always arrive at index 3 no matter what. Its maximum jump length is 0, which makes it impossible to reach the last index.

Constraints:

    1 <= nums.length <= 3 * 10^4
    0 <= nums[i][j] <= 10^5
*/

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let len = nums.len();
        let mut jumppalbe_rng = 0;
        for (i, jump) in nums.into_iter().enumerate().take(len.saturating_sub(1)) {
            if i > jumppalbe_rng {
                return false;
            }
            jumppalbe_rng = jumppalbe_rng.max(i + jump as usize);
        }
        jumppalbe_rng >= len.saturating_sub(1)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    #[test]
    fn test_jump_possible() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
    }

    #[test]
    fn test_jump_impossible() {
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
    }

    #[test]
    fn test_jump_empty_and_single() {
        assert_eq!(Solution::can_jump(vec![0]), true);
        assert_eq!(Solution::can_jump(Vec::new()), true);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::can_jump(vec![1, 2, 4, 7, 8, 9, 10]));
    }
}
