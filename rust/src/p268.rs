/*
Problem 268. Missing Number
===========================

https://leetcode.com/problems/missing-number/

Given an array nums containing n distinct numbers in the range [0, n], return the only number in
the range that is missing from the array.

Follow up: Could you implement a solution using only O(1) extra space complexity and O(n) runtime
complexity?

Example 1:

    Input: nums = [3,0,1]
    Output: 2
    Explanation: n = 3 since there are 3 numbers, so all numbers are in the range [0,3]. 2 is the
                 missing number in the range since it does not appear in nums.

Example 2:

    Input: nums = [0,1]
    Output: 2
    Explanation: n = 2 since there are 2 numbers, so all numbers are in the range [0,2]. 2 is the
                 missing number in the range since it does not appear in nums.

Example 3:

    Input: nums = [9,6,4,2,3,5,7,0,1]
    Output: 8
    Explanation: n = 9 since there are 9 numbers, so all numbers are in the range [0,9]. 8 is the
                 missing number in the range since it does not appear in nums.

Example 4:

    Input: nums = [0]
    Output: 1
    Explanation: n = 1 since there is 1 number, so all numbers are in the range [0,1]. 1 is the
                 missing number in the range since it does not appear in nums.

Constraints:

    - n == nums.length
    - 1 <= n <= 104
    - 0 <= nums[i] <= n
    - All the numbers of nums are unique.
*/

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let set = (1..=nums.len()).fold(0, |result, n| result ^ n);
        nums.into_iter().fold(set, |rest, n| n as usize ^ rest) as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_miss2() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
    }

    #[test]
    fn test_miss8() {
        assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }

    #[test]
    fn test_miss1() {
        assert_eq!(Solution::missing_number(vec![0]), 1);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]));
    }
}
