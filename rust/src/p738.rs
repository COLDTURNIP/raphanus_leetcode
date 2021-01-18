/*
Problem 738. Monotone Increasing Digits
=======================================

https://leetcode.com/problems/monotone-increasing-digits/

Given a non-negative integer N, find the largest number that is less than or equal to N with
monotone increasing digits.

(Recall that an integer has monotone increasing digits if and only if each pair of adjacent digits
x and y satisfy x <= y.)

Example 1:
    Input: N = 10
    Output: 9

Example 2:
    Input: N = 1234
    Output: 1234

Example 3:
    Input: N = 332
    Output: 299

Note: N is an integer in the range [0, 10^9].
*/

impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut last = n % 10;
        let mut mask = 10;
        let mut rest = n / 10;
        let mut ans = n;
        while rest > 0 {
            let cur = rest % 10;
            rest /= 10;
            if cur > last {
                ans -= ans % mask + 1;
                last = cur - 1;
            } else {
                last = cur;
            }
            mask *= 10;
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
    fn test_10() {
        assert_eq!(Solution::monotone_increasing_digits(10), 9);
    }

    #[test]
    fn test_1234() {
        assert_eq!(Solution::monotone_increasing_digits(1234), 1234);
    }

    #[test]
    fn test_332() {
        assert_eq!(Solution::monotone_increasing_digits(332), 299);
    }

    #[test]
    fn test_zero() {
        assert_eq!(Solution::monotone_increasing_digits(0), 0);
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(Solution::monotone_increasing_digits(9), 9);
        assert_eq!(Solution::monotone_increasing_digits(7), 7);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::monotone_increasing_digits(1294));
    }
}
