/*
Problem 66. Plus One
====================

https://leetcode.com/problems/plus-one/

Given a non-empty array of digits representing a non-negative integer, increment one to the
integer.

The digits are stored such that the most significant digit is at the head of the list, and each
element in the array contains a single digit.

You may assume the integer does not contain any leading zero, except the number 0 itself.

Example 1:

    Input: digits = [1,2,3]
    Output: [1,2,4]
    Explanation: The array represents the integer 123.

Example 2:

    Input: digits = [4,3,2,1]
    Output: [4,3,2,2]
    Explanation: The array represents the integer 4321.

Example 3:

    Input: digits = [0]
    Output: [1]

Constraints:

    1 <= digits.length <= 100
    0 <= digits[i] <= 9
*/

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let len = digits.len();
        let mut ans = digits;
        let mut carry = 1;
        if len > 0 {
            for i in (0..=len - 1).rev() {
                let sum = ans[i] + carry;
                carry = sum / 10;
                ans[i] = sum % 10;
                if carry == 0 {
                    break;
                }
            }
        }
        if carry > 0 {
            ans.insert(0, carry)
        }
        ans
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    #[test]
    fn test_123() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::plus_one(Vec::new()), vec![1]);
    }

    #[test]
    fn test_zero() {
        assert_eq!(Solution::plus_one(vec![0]), vec![1]);
    }

    #[test]
    fn test_9999() {
        assert_eq!(Solution::plus_one(vec![9, 9, 9, 9]), vec![1, 0, 0, 0, 0]);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::plus_one(vec![9, 9, 9, 9, 9]));
    }
}
