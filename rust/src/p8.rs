/*
Problem 8. String to Integer (atoi)
============================

https://leetcode.com/problems/string-to-integer-atoi/

Implement atoi which converts a string to an integer.

The function first discards as many whitespace characters as necessary until the first
non-whitespace character is found. Then, starting from this character, takes an optional initial
plus or minus sign followed by as many numerical digits as possible, and interprets them as a
numerical value.

The string can contain additional characters after those that form the integral number, which are
ignored and have no effect on the behavior of this function.

If the first sequence of non-whitespace characters in str is not a valid integral number, or if no
such sequence exists because either str is empty or it contains only whitespace characters, no
conversion is performed.

If no valid conversion could be performed, a zero value is returned.

Note:

    Only the space character ' ' is considered as whitespace character. Assume we are dealing with
    an environment which could only store integers within the 32-bit signed integer range: [−231,
    231 − 1]. If the numerical value is out of the range of representable values, INT_MAX (231 − 1)
    or INT_MIN (−231) is returned.

Example 1:

    Input: "42"
    Output: 42

Example 2:

    Input: "   -42"
    Output: -42
    Explanation: The first non-whitespace character is '-', which is the minus sign.
                 Then take as many numerical digits as possible, which gets 42.

Example 3:

    Input: "4193 with words"
    Output: 4193
    Explanation: Conversion stops at digit '3' as the next character is not a numerical digit.

Example 4:

    Input: "words and 987"
    Output: 0
    Explanation: The first non-whitespace character is 'w', which is not a numerical
                 digit or a +/- sign. Therefore no valid conversion could be performed.

Example 5:

    Input: "-91283472332"
    Output: -2147483648
    Explanation: The number "-91283472332" is out of the range of a 32-bit signed integer.
                 Thefore INT_MIN (−231) is returned.

*/

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut sign = None;
        let mut ans: i32 = 0;
        for c in s.chars() {
            match (c, c.to_digit(10)) {
                (_, Some(n)) => {
                    if sign == None {
                        sign = Some(1);
                    }
                    ans = ans
                        .saturating_mul(10)
                        .saturating_add(n as i32 % 10 * sign.unwrap());
                }
                (' ', _) if sign == None => {}
                ('+', _) if sign == None => sign = Some(1),
                ('-', _) if sign == None => sign = Some(-1),
                _ => break,
            };
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
    fn test_42() {
        assert_eq!(Solution::my_atoi("42".to_owned()), 42);
    }

    #[test]
    fn test_positive() {
        assert_eq!(Solution::my_atoi("+1".to_owned()), 1);
    }

    #[test]
    fn test_prefix_space() {
        assert_eq!(Solution::my_atoi("    -42".to_owned()), -42);
    }

    #[test]
    fn test_with_words() {
        assert_eq!(Solution::my_atoi("4193 with words".to_owned()), 4193);
    }

    #[test]
    fn test_start_with_words() {
        assert_eq!(Solution::my_atoi("words and 987".to_owned()), 0);
    }

    #[test]
    fn test_underflow() {
        assert_eq!(Solution::my_atoi("-91283472332".to_owned()), -2147483648);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::my_atoi("     -91283472332 ljlqwrjklkj".to_owned()))
    }
}
