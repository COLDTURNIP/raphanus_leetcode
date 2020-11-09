/*
Problem 989. Add to Array-Form of Integer
=========================================

https://leetcode.com/problems/add-to-array-form-of-integer/

For a non-negative integer X, the array-form of X is an array of its digits in left to right order.
For example, if X = 1231, then the array form is [1,2,3,1].

Given the array-form A of a non-negative integer X, return the array-form of the integer X+K.

Example 1:

    Input: A = [1,2,0,0], K = 34
    Output: [1,2,3,4]
    Explanation: 1200 + 34 = 1234

Example 2:

    Input: A = [2,7,4], K = 181
    Output: [4,5,5]
    Explanation: 274 + 181 = 455

Example 3:

    Input: A = [2,1,5], K = 806
    Output: [1,0,2,1]
    Explanation: 215 + 806 = 1021

Example 4:

    Input: A = [9,9,9,9,9,9,9,9,9,9], K = 1
    Output: [1,0,0,0,0,0,0,0,0,0,0]
    Explanation: 9999999999 + 1 = 10000000000

Note：

    - 1 <= A.length <= 10000
    - 0 <= A[i] <= 9
    - 0 <= K <= 10000
    - If A.length > 1, then A[0] != 0
*/

use std::collections::VecDeque;

impl Solution {
    pub fn add_to_array_form(a: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ans: VecDeque<i32> = a.into();
        let mut rest = k;
        let mut carry = 0;
        for n in ans.iter_mut().rev() {
            if rest == 0 && carry == 0 {
                break;
            }
            let cur_sum = *n + rest % 10 + carry;
            *n = cur_sum % 10;
            carry = cur_sum / 10;
            rest /= 10;
        }
        while rest > 0 || carry > 0 {
            let cur_sum = rest % 10 + carry;
            ans.push_front(cur_sum % 10);
            carry = cur_sum / 10;
            rest /= 10;
        }
        ans.into()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_1200_34() {
        assert_eq!(
            Solution::add_to_array_form(vec![1, 2, 0, 0], 34),
            vec![1, 2, 3, 4]
        );
    }

    #[test]
    fn test_274_181() {
        assert_eq!(
            Solution::add_to_array_form(vec![2, 7, 4], 181),
            vec![4, 5, 5]
        );
    }

    #[test]
    fn test_215_806() {
        assert_eq!(
            Solution::add_to_array_form(vec![2, 1, 5], 806),
            vec![1, 0, 2, 1]
        );
    }

    #[test]
    fn test_9999999999_1() {
        assert_eq!(
            Solution::add_to_array_form(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9], 1),
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::add_to_array_form(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9], 1));
    }
}
