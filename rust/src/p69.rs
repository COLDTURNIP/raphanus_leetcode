/*
Problem 69. Sqrt(x)
===================

https://leetcode.com/problems/sqrtx/

Implement int sqrt(int x).

Compute and return the square root of x, where x is guaranteed to be a non-negative integer.

Since the return type is an integer, the decimal digits are truncated and only the integer part of
the result is returned.

Example 1:

    Input: 4
    Output: 2

Example 2:

    Input: 8
    Output: 2

Explanation: The square root of 8 is 2.82842..., and since
             the decimal part is truncated, 2 is returned.
*/

use std::cmp::Ordering::{Equal, Greater, Less};

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x <= 0 {
            return 0;
        }
        // Classical binary search.
        // TODO: change to some other hack for speed, e.g. Newton's method
        let (mut lb, mut rb) = (1, x);
        while lb < rb {
            let mid = lb.saturating_add(rb).saturating_add(1) / 2;
            match mid.checked_mul(mid).map_or(Greater, |n2| n2.cmp(&x)) {
                Greater => rb = mid - 1,
                Equal => return mid,
                Less => lb = mid,
            }
        }
        lb
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_sqrt_4() {
        assert_eq!(Solution::my_sqrt(4), 2);
    }

    #[test]
    fn test_sqrt_8() {
        assert_eq!(Solution::my_sqrt(8), 2);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::my_sqrt(std::i32::MAX));
    }
}
