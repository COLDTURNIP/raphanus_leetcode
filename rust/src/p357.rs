/*
Problem 357. Count Numbers with Unique Digits
=============================================

https://leetcode.com/problems/count-numbers-with-unique-digits/

Given a non-negative integer n, count all numbers with unique digits, x, where 0 ≤ x < 10^n.

Example:

    Input: 2
    Output: 91
    Explanation: The answer should be the total numbers in the range of 0 ≤ x < 100,
                 excluding 11,22,33,44,55,66,77,88,99

Constraints:

    0 <= n <= 8
*/

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        assert!(n >= 0 && n <= 8);
        let mut cnt = 1; // the zero
        let mut pn1 = 1;
        let mut c9n1 = 1;
        for digit_num in 1..=n {
            // All permutation except starting by zero:
            //   - Permutation without zero: C(9, n) * P(n)
            //   - Permutation with zero: C(9, n-1) * (P(n) - P(n-1))
            let pn = pn1 * digit_num;
            let c9n = c9n1 * (9 - digit_num + 1) / digit_num;
            cnt += c9n * pn + c9n1 * (pn - pn1);
            pn1 = pn;
            c9n1 = c9n;
        }
        cnt
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case2() {
        assert_eq!(Solution::count_numbers_with_unique_digits(2), 91);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::count_numbers_with_unique_digits(8));
    }
}
