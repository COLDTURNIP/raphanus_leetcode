/*
Problem 678. Valid Parenthesis String
=====================================

https://leetcode.com/problems/valid-parenthesis-string/

Given a string containing only three types of characters: '(', ')' and '*', write a function to
check whether this string is valid. We define the validity of a string by these rules:

    - Any left parenthesis '(' must have a corresponding right parenthesis ')'.
    - Any right parenthesis ')' must have a corresponding left parenthesis '('.
    - Left parenthesis '(' must go before the corresponding right parenthesis ')'.
    - '*' could be treated as a single right parenthesis ')' or a single left parenthesis '(' or an
      empty string.
    - An empty string is also valid.

Example 1:

    Input: "()"
    Output: True

Example 2:

    Input: "(*)"
    Output: True

Example 3:
    Input: "(*))"
    Output: True

Note:
    - The string size will be in the range [1, 100].
*/

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        // step 1. check treat all stars as left parenthesis and count
        let mut left_cnt = 0;
        for c in s.chars() {
            match c {
                '(' | '*' => left_cnt += 1,
                ')' => left_cnt -= 1,
                _ => unreachable!(),
            }
            if left_cnt < 0 {
                return false;
            }
        }

        // step 2. check treat all stars as right parenthesis and count
        let mut right_cnt = 0;
        for c in s.chars().rev() {
            match c {
                '(' => right_cnt -= 1,
                ')' | '*' => right_cnt += 1,
                _ => unreachable!(),
            }
            if right_cnt < 0 {
                return false;
            }
        }

        // step 3. both way are valid, impossible to be invalid
        true
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert!(Solution::check_valid_string(String::from("()")));
    }

    #[test]
    fn test_case2() {
        assert!(Solution::check_valid_string(String::from("(*)")));
    }

    #[test]
    fn test_case3() {
        assert!(Solution::check_valid_string(String::from("(*))")));
    }

    #[test]
    fn test_case4() {
        assert!(!Solution::check_valid_string(String::from("(*)))")));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::check_valid_string(String::from("(*(()((*)))")));
    }
}
