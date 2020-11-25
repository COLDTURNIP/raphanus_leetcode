/*
Problem 343. Integer Break
==========================

https://leetcode.com/problems/integer-break/

Given a positive integer n, break it into the sum of at least two positive integers and maximize
the product of those integers. Return the maximum product you can get.

Example 1:

    Input: 2
    Output: 1
    Explanation: 2 = 1 + 1, 1 × 1 = 1.

Example 2:

    Input: 10
    Output: 36
    Explanation: 10 = 3 + 3 + 4, 3 × 3 × 4 = 36.

Note: You may assume that n is not less than 2 and not larger than 58.
*/

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        assert!(n >= 2 && n <= 58);
        let mut ans = 0;
        for i in 2.. {
            let part = n / i;
            if part == 0 {
                break;
            };
            let rem = n % i;
            let cur_ans = (part + 1).pow(rem as u32) * part.pow((i - rem) as u32);
            if cur_ans > ans {
                ans = cur_ans;
            } else {
                break;
            }
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
    fn test_2() {
        // 2 = 1 + 1
        assert_eq!(Solution::integer_break(2), 1);
    }

    #[test]
    fn test_8() {
        // 8 = 3 + 3 + 2
        assert_eq!(Solution::integer_break(8), 18);
    }

    #[test]
    fn test_10() {
        // 10 = 4 + 3 + 3
        assert_eq!(Solution::integer_break(10), 36);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::integer_break(58));
    }
}
