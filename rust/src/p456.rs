/*
Problem 456. 132 Pattern
========================

https://leetcode.com/problems/132-pattern/

Given an array of n integers nums, a 132 pattern is a subsequence of three integers nums[i],
nums[j] and nums[k] such that i < j < k and nums[i] < nums[k] < nums[j].

Return true if there is a 132 pattern in nums, otherwise, return false.

Follow up: The O(n^2) is trivial, could you come up with the O(n logn) or the O(n) solution?

Example 1:

    Input: nums = [1,2,3,4]
    Output: false
    Explanation: There is no 132 pattern in the sequence.

Example 2:

    Input: nums = [3,1,4,2]
    Output: true
    Explanation: There is a 132 pattern in the sequence: [1, 4, 2].

Example 3:

    Input: nums = [-1,3,2,0]
    Output: true
    Explanation: There are three 132 patterns in the sequence: [-1, 3, 2], [-1, 3, 0] and [-1, 2, 0].

Constraints:

    - n == nums.length
    - 1 <= n <= 10^4
    - -10^9 <= nums[i] <= 10^9
*/

use std::cmp::Ordering::{Greater, Less};

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        // trick: need a stack to store n2 candidates. Since we scan nums backward, we can use the
        // tail of the nums as the stack.
        let mut nums = nums;
        let len = nums.len();
        let mut n3 = std::i32::MIN;
        let mut n2_candidate_idx = nums.len();
        for i in (0..len).rev() {
            let n = nums[i];
            match n.cmp(&n3) {
                Less => return true,
                Greater => {
                    while let Some(&new_n3) = nums.get(n2_candidate_idx).filter(|&&n2| n > n2) {
                        // pop the n2 as n3, push the n as new n2
                        n3 = new_n3;
                        n2_candidate_idx += 1;
                    }
                    n2_candidate_idx -= 1;
                    nums[n2_candidate_idx] = n;
                }
                _ => {}
            }
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
    fn test_case1() {
        assert!(!Solution::find132pattern(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_case2() {
        assert!(Solution::find132pattern(vec![3, 1, 4, 2]));
    }

    #[test]
    fn test_case3() {
        assert!(Solution::find132pattern(vec![-1, 3, 2, 0]));
    }

    #[test]
    fn test_case4() {
        assert!(Solution::find132pattern(vec![
            1, 3, 2, 4, 5, 6, 7, 8, 9, 10
        ]));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::find132pattern(vec![109, 1, 9, 19, 99, -2]));
    }
}
