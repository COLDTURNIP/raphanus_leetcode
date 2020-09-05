/*
Problem 13. Roman to Integer
============================

https://leetcode.com/problems/roman-to-integer/

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

Given a roman numeral, convert it to an integer. Input is guaranteed to be within the range from 1
to 3999.

Example 1:

    Input: "III"
    Output: 3

Example 2:

    Input: "IV"
    Output: 4

Example 3:

    Input: "IX"
    Output: 9

Example 4:

    Input: "LVIII"
    Output: 58
    Explanation: L = 50, V= 5, III = 3.

Example 5:

    Input: "MCMXCIV"
    Output: 1994
    Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
*/

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut ans = 0;
        let mut next_iter = s.chars().map(Self::char_to_digit);
        let mut cur = next_iter.next();
        loop {
            let next = next_iter.next();
            ans += match (cur, next) {
                (Some(n1), Some(n2)) => {
                    if n1 < n2 {
                        -n1
                    } else {
                        n1
                    }
                }
                (Some(n), None) => n,
                _ => break,
            };
            cur = next;
        }
        ans
    }

    fn char_to_digit(c: char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => unreachable!("invalid char '{}'", c),
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_3() {
        assert_eq!(Solution::roman_to_int("III".to_owned()), 3);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::roman_to_int("IV".to_owned()), 4);
    }

    #[test]
    fn test_9() {
        assert_eq!(Solution::roman_to_int("IX".to_owned()), 9);
    }

    #[test]
    fn test_58() {
        assert_eq!(Solution::roman_to_int("LVIII".to_owned()), 58);
    }

    #[test]
    fn test_1994() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_owned()), 1994);
    }

    #[test]
    #[should_panic]
    fn test_invalid() {
        Solution::roman_to_int("  MCMXCIV".to_owned());
    }


    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::roman_to_int("MCMXCIV".to_owned()))
    }
}
