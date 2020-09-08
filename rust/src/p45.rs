/*
Problem 45. Jump Game II
==================================

https://leetcode.com/problems/jump-game-ii/

Given an array of non-negative integers, you are initially positioned at the first index of the
array.

Each element in the array represents your maximum jump length at that position.

Your goal is to reach the last index in the minimum number of jumps.

Example:

Input: [2,3,1,1,4]
Output: 2
Explanation: The minimum number of jumps to reach the last index is 2.
    Jump 1 step from index 0 to 1, then 3 steps to the last index.
Note:

You can assume that you can always reach the last index.
*/

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        // Greedy algorithm. This does not compute the exact jump path.
        let len = nums.len();
        let mut jumpped_rng = 0;
        let mut jumppable_rng = 0;
        let mut cnt = 0;
        for (i, rng) in nums.iter().map(|&r| r as usize).enumerate().take(len - 1) {
            jumppable_rng = jumppable_rng.max(i + rng);
            if jumppable_rng >= len - 1 {
                // early stop
                cnt += 1;
                break;
            }
            if jumpped_rng == i {
                // Reaching the current jump boundary, extend to the maximum jumppable range.
                cnt += 1;
                jumpped_rng = jumppable_rng;
            }
        }
        cnt
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    #[test]
    fn test_jump_2() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn test_jump_2_with_zero() {
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }

    //#[bench]
    //fn bench(b: &mut test::Bencher) {
    //    b.iter(|| Solution::jump(vec![1, 2, 4, 7, 8, 9, 10]));
    //}
}
