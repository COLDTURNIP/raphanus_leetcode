/*
Problem 9. Palindrome Number
============================

https://leetcode.com/problems/palindrome-number/

Determine whether an integer is a palindrome. An integer is a palindrome when it reads the same
backward as forward.

Example 1:

    Input: 121
    Output: true

Example 2:

    Input: -121
    Output: false
    Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore
                 it is not a palindrome.

Example 3:

    Input: 10
    Output: false
    Explanation: Reads 01 from right to left. Therefore it is not a palindrome.

Follow up:

    Coud you solve it without converting the integer to a string?
*/

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x >= 0 {
            let mut rest = x;
            let mut rev = 0;
            while rest > 0 {
                rev = rev * 10 + rest % 10;
                rest /= 10;
            }
            x == rev
        } else {
            false
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_121() {
        assert_eq!(Solution::is_palindrome(121), true);
    }

    #[test]
    fn test_negative() {
        assert_eq!(Solution::is_palindrome(-121), false);
    }

    #[test]
    fn test_false() {
        assert_eq!(Solution::is_palindrome(10), false);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::is_palindrome(1233321))
    }
}
