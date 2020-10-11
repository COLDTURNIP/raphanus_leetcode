/*
Problem 167. Two Sum II - Input array is sorted
===============================================

https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/

Given an array of integers that is already sorted in ascending order, find two numbers such that
they add up to a specific target number.

The function twoSum should return indices of the two numbers such that they add up to the target,
where index1 must be less than index2.

Note:

    - Your returned answers (both index1 and index2) are not zero-based.
    - You may assume that each input would have exactly one solution and you may not use the same
      element twice.

Example 1:

    Input: numbers = [2,7,11,15], target = 9
    Output: [1,2]
    Explanation: The sum of 2 and 7 is 9. Therefore index1 = 1, index2 = 2.

Example 2:

    Input: numbers = [2,3,4], target = 6
    Output: [1,3]

Example 3:

    Input: numbers = [-1,0], target = -1
    Output: [1,2]

Constraints:

    - 2 <= nums.length <= 3 * 104
    - -1000 <= nums[i] <= 1000
    - nums is sorted in increasing order.
    - -1000 <= target <= 1000
*/

use std::cmp::Ordering::{Equal, Greater, Less};

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let len = numbers.len();
        let (mut lb, mut rb) = (0, len - 1);
        while lb < rb {
            match target.cmp(&(numbers[lb] + numbers[rb])) {
                Less => rb -= 1,
                Greater => lb += 1,
                Equal => return vec![lb as i32 + 1, rb as i32 + 1],
            };
        }
        unreachable!()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_find_9() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }

    #[test]
    fn test_find_6() {
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    }

    #[test]
    fn test_find_neg_1() {
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::two_sum(vec![2, 7, 11, 15], 9));
    }
}
