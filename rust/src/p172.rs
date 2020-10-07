/*
Problem 172. Factorial Trailing Zeroes
======================================

https://leetcode.com/problems/factorial-trailing-zeroes/

Given an integer n, return the number of trailing zeroes in n!.

Follow up: Could you write a solution that works in logarithmic time complexity?



Example 1:

Input: n = 3
Output: 0
Explanation: 3! = 6, no trailing zero.
Example 2:

Input: n = 5
Output: 1
Explanation: 5! = 120, one trailing zero.
Example 3:

Input: n = 0
Output: 0


Constraints:

1 <= n <= 10^4
*/

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut num2 = 0;
        let mut num5 = 0;
        for mut i in 2..=n {
            num2 += i.trailing_zeros();
            while i % 5 == 0 {
                num5 += 1;
                i /= 5;
            }
        }
        num2.min(num5) as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_3() {
        assert_eq!(Solution::trailing_zeroes(3), 0);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::trailing_zeroes(5), 1);
    }

    #[test]
    fn test_zero() {
        assert_eq!(Solution::trailing_zeroes(0), 0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::trailing_zeroes(65243);
        });
    }
}
