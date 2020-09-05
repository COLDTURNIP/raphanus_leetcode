/*
Problem 17. Letter Combinations of a Phone Number
========================

https://leetcode.com/problems/letter-combinations-of-a-phone-number/

Given a string containing digits from 2-9 inclusive, return all possible letter combinations that
the number could represent.

A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does
not map to any letters.

    1    2    3
    o_o  abc  def

    4    5    6
    ghi  jkl  mno

    7    8    9
    pqrs tuv  wxyz

    *    0    ^
    +    _    #

Example:

    Input: "23"
    Output: ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].

Note:

    Although the above answer is in lexicographical order, your answer could be in any order you
    want.
*/

use std::collections::VecDeque;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }
        let digit_vec = digits
            .chars()
            .map(Self::digit_to_chars)
            .collect::<Vec<&str>>();
        let mut ans = VecDeque::with_capacity(digit_vec.iter().map(|s| s.len()).product());
        ans.push_back("".to_owned());
        for (len, chars) in digit_vec.into_iter().enumerate() {
            while ans.front().unwrap().len() < len + 1 {
                let orig = ans.pop_front().unwrap();
                for c in chars.chars() {
                    let mut extend = orig.clone();
                    extend.push(c);
                    ans.push_back(extend);
                }
            }
        }
        ans.into()
    }

    fn digit_to_chars(digit: char) -> &'static str {
        match digit {
            '2' => "abc",
            '3' => "def",
            '4' => "hgi",
            '5' => "jkl",
            '6' => "mno",
            '7' => "pqrs",
            '8' => "tuv",
            '9' => "wxyz",
            _ => unreachable!(),
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;
    use std::collections::HashSet;

    #[test]
    fn test_23() {
        assert_eq!(
            Solution::letter_combinations("23".to_owned())
                .into_iter()
                .collect::<HashSet<String>>(),
            vec![
                "ad".to_owned(),
                "ae".to_owned(),
                "af".to_owned(),
                "bd".to_owned(),
                "be".to_owned(),
                "bf".to_owned(),
                "cd".to_owned(),
                "ce".to_owned(),
                "cf".to_owned()
            ]
            .into_iter()
            .collect::<HashSet<String>>()
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(
            Solution::letter_combinations("".to_owned()),
            Vec::<String>::new()
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::letter_combinations("237".to_owned()))
    }
}
