/*
Problem 7. Reverse Integer
============================

https://leetcode.com/problems/reverse-integer/

Given a 32-bit signed integer, reverse digits of an integer.

Example 1:

    Input: 123
    Output: 321

Example 2:

    Input: -123
    Output: -321

Example 3:

    Input: 120
    Output: 21

Note:
    Assume we are dealing with an environment which could only store integers within the 32-bit
    signed integer range: [−231,  231 − 1]. For the purpose of this problem, assume that your
    function returns 0 when the reversed integer overflows.
*/

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut ans = 0_i32;
        let mut rest = x.abs();
        let sign = (x > 0) as i32 * 2 - 1;
        while rest > 0 {
            match ans
                .checked_mul(10)
                .and_then(|n| n.checked_add(rest % 10 * sign))
            {
                Some(n) => ans = n,
                None => return 0,
            }
            rest /= 10;
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
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn test_neg1234() {
        assert_eq!(Solution::reverse(-1234), -4321);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::reverse(-1234));
    }
}
