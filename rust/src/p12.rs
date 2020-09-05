/*
Problem 12. Integer to Roman
============================

https://leetcode.com/problems/integer-to-roman/

Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

    Symbol       Value
    I             1
    V             5
    X             10
    L             50
    C             100
    D             500
    M             1000

For example, two is written as II in Roman numeral, just two one's added together. Twelve is
written as, XII, which is simply X + II. The number twenty seven is written as XXVII, which is XX +
V + II.

Roman numerals are usually written largest to smallest from left to right. However, the numeral for
four is not IIII. Instead, the number four is written as IV. Because the one is before the five we
subtract it making four. The same principle applies to the number nine, which is written as IX.
There are six instances where subtraction is used:

    I can be placed before V (5) and X (10) to make 4 and 9.
    X can be placed before L (50) and C (100) to make 40 and 90.
    C can be placed before D (500) and M (1000) to make 400 and 900.

Given an integer, convert it to a roman numeral. Input is guaranteed to be within the range from 1 to 3999.

Example 1:

    Input: 3
    Output: "III"

Example 2:

    Input: 4
    Output: "IV"

Example 3:

    Input: 9
    Output: "IX"

Example 4:

    Input: 58
    Output: "LVIII"
    Explanation: L = 50, V = 5, III = 3.

Example 5:

    Input: 1994
    Output: "MCMXCIV"
    Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
*/

struct DigitChar {
    five: char,
    one: char,
}

static DIGIT_CHAR_VEC: &[DigitChar] = &[
    DigitChar {
        five: 'V',
        one: 'I',
    },
    DigitChar {
        five: 'L',
        one: 'X',
    },
    DigitChar {
        five: 'D',
        one: 'C',
    },
    DigitChar {
        five: '?',
        one: 'M',
    },
];

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        assert!(num < 4000);
        let mut dcv_i = 0;
        let mut rest = num;
        let mut ans = Vec::<char>::with_capacity(4 * 4);
        while rest > 0 {
            let digit = rest % 10;
            match digit {
                1..=3 => (0..digit).for_each(|_| ans.push(DIGIT_CHAR_VEC[dcv_i].one)),
                4 => {
                    ans.push(DIGIT_CHAR_VEC[dcv_i].five);
                    ans.push(DIGIT_CHAR_VEC[dcv_i].one);
                }
                5..=8 => {
                    (5..digit).for_each(|_| ans.push(DIGIT_CHAR_VEC[dcv_i].one));
                    ans.push(DIGIT_CHAR_VEC[dcv_i].five);
                }
                9 => {
                    ans.push(DIGIT_CHAR_VEC[dcv_i + 1].one);
                    ans.push(DIGIT_CHAR_VEC[dcv_i].one);
                }
                _ => {}
            }
            rest /= 10;
            dcv_i += 1;
        }
        ans.into_iter().rev().collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    #[test]
    fn test_3() {
        assert_eq!(Solution::int_to_roman(3), "III".to_owned());
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::int_to_roman(4), "IV".to_owned());
    }

    #[test]
    fn test_9() {
        assert_eq!(Solution::int_to_roman(9), "IX".to_owned());
    }

    #[test]
    fn test_10() {
        assert_eq!(Solution::int_to_roman(10), "X".to_owned());
    }

    #[test]
    fn test_58() {
        assert_eq!(Solution::int_to_roman(58), "LVIII".to_owned());
    }

    #[test]
    fn test_1994() {
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_owned());
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn test_invalid() {
        Solution::int_to_roman(4000);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::int_to_roman(1994))
    }
}
