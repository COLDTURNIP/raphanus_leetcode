/*
665. Non-decreasing Array
=========================

https://leetcode.com/problems/non-decreasing-array/

Given an array nums with n integers, your task is to check if it could become non-decreasing by
modifying at most 1 element.

We define an array is non-decreasing if nums[i] <= nums[i + 1] holds for every i (0-based) such
that (0 <= i <= n - 2).



Example 1:

    Input: nums = [4,2,3]
    Output: true
    Explanation: You could modify the first 4 to 1 to get a non-decreasing array.

Example 2:

    Input: nums = [4,2,1]
    Output: false
    Explanation: You can't get a non-decreasing array by modify at most one element.

Constraints:

    1 <= n <= 10 ^ 4
    - 10 ^ 5 <= nums[i] <= 10 ^ 5
*/

impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let len = nums.len();
        let low = (0..len - 1).find(|&i| nums[i] > nums[i + 1]);
        let high = (1..len).map(|j| len - j).find(|&j| nums[j - 1] > nums[j]);
        match (low, high) {
            (Some(i), Some(j)) => {
                if i + 1 == j {
                    let prev = i.checked_sub(1).map(|i| nums[i]).unwrap_or(std::i32::MIN);
                    let next = nums.get(j + 1).cloned().unwrap_or(std::i32::MAX);
                    prev <= nums[i + 1] || nums[j - 1] <= next
                } else {
                    false
                }
            }
            _ => true,
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
        assert_eq!(Solution::check_possibility(vec![4, 2, 3]), true);
        assert_eq!(Solution::check_possibility(vec![4, 2, 1]), false);
        assert_eq!(Solution::check_possibility(vec![3, 4, 2, 3]), false);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::check_possibility(vec![1, 2, 3, 4]));
    }
}
