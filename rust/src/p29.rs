/*
Problem 29. Divide Two Integers
===============================

https://leetcode.com/problems/divide-two-integers/

Given two integers dividend and divisor, divide two integers without using multiplication, division
and mod operator.

Return the quotient after dividing dividend by divisor.

The integer division should truncate toward zero, which means losing its fractional part. For
example, truncate(8.345) = 8 and truncate(-2.7335) = -2.

Example 1:

    Input: dividend = 10, divisor = 3
    Output: 3
    Explanation: 10/3 = truncate(3.33333..) = 3.

Example 2:

    Input: dividend = 7, divisor = -3
    Output: -2
    Explanation: 7/-3 = truncate(-2.33333..) = -2.

Note:

    Both dividend and divisor will be 32-bit signed integers.

    The divisor will never be 0.

    Assume we are dealing with an environment which could only store integers within the 32-bit
    signed integer range: [−231,  231 − 1]. For the purpose of this problem, assume that your
    function returns 231 − 1 when the division result overflows.
*/

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        assert_ne!(divisor, 0);
        if dividend == std::i32::MIN && divisor == -1 {
            return std::i32::MAX;
        } else if dividend == 0 {
            return 0;
        }
        // main idea:
        // - Bitwise operation to simulate *2 multiply
        // - Reverse positive to negative to prevent overflow problem
        // - TODO: i32.leading_ones is nightly-only currently. Caluated with leading_zeros instead.
        let sign = dividend.signum() == divisor.signum();
        let divisor = if divisor > 0 { -divisor } else { divisor };
        let mut rest = if dividend > 0 { -dividend } else { dividend };
        let max_shift = ((!divisor as u32).leading_zeros() - 1)
            .saturating_sub((!rest as u32).leading_zeros() - 1);
        let mut ans = 0;
        for shift in (0..=max_shift).rev() {
            let sub = divisor << shift;
            ans = (ans << 1)
                + if rest <= sub {
                    rest -= sub;
                    1
                } else {
                    0
                };
        }
        if sign {
            ans
        } else {
            -ans
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    #[test]
    fn test_10_3() {
        assert_eq!(Solution::divide(10, 3), 3);
    }

    #[test]
    fn test_1_2() {
        assert_eq!(Solution::divide(1, 2), 0);
    }

    #[test]
    fn test_7_neg3() {
        assert_eq!(Solution::divide(7, -3), -2);
    }

    #[test]
    fn test_overflow() {
        assert_eq!(Solution::divide(std::i32::MIN, -1), std::i32::MAX);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::divide(17234, 98));
    }
}
