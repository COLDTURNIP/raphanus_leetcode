/*
Problem 50. Pow(x, n)
=====================

https://leetcode.com/problems/powx-n/

Implement pow(x, n), which calculates x raised to the power n (i.e. xn).

Example 1:

    Input: x = 2.00000, n = 10
    Output: 1024.00000

Example 2:

    Input: x = 2.10000, n = 3
    Output: 9.26100

Example 3:

    Input: x = 2.00000, n = -2
    Output: 0.25000
    Explanation: 2^-2 = 1/2^2 = 1/4 = 0.25

Constraints:

    -100.0 < x < 100.0
    -2^31 <= n <= 2^31-1
    -10^4 <= x^n <= 10^4
*/

use std::f64::EPSILON;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if x.abs() < EPSILON {
            return 0.0;
        } else if (x - 1.0).abs() < EPSILON || n == 0 {
            return 1.0;
        }
        let mut ans = 1.0;
        let mut rest = if n > 0 { n as u32 } else { !(n as u32) + 1 };
        let mut pow = if n.is_positive() { x } else { 1.0 / x };
        while rest > 0 {
            if rest & 0x01 == 1 {
                ans *= pow;
            }
            pow *= pow;
            rest >>= 1;
        }
        ans
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    fn check(x: f64, n: i32, expect: f64) {
        let ans = Solution::my_pow(x, n);
        assert!(
            (ans - expect).abs() < std::f64::EPSILON,
            "ans = {}, expect = {}",
            ans,
            expect
        );
    }

    #[test]
    fn test_2_10() {
        check(2.0, 10, 1024.0);
    }

    #[test]
    fn test_21_3() {
        check(2.1, 3, 9.261);
    }

    #[test]
    fn test_negative() {
        check(2.0, -2, 0.25);
    }

    #[test]
    fn test_neg_min() {
        check(2.0, -2147483648, 0.0);
    }

    #[test]
    fn test_zero() {
        check(2.0, 0, 1.0);
        check(0.0, 8, 0.0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::my_pow(2.0, 10));
    }
}
