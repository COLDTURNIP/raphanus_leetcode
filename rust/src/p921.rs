/*
Problem 921. Minimum Add to Make Parentheses Valid
==================================================

Given a string S of '(' and ')' parentheses, we add the minimum number of parentheses ( '(' or ')',
and in any positions ) so that the resulting parentheses string is valid.

Formally, a parentheses string is valid if and only if:

    - It is the empty string, or
    - It can be written as AB (A concatenated with B), where A and B are valid strings, or
    - It can be written as (A), where A is a valid string.

Given a parentheses string, return the minimum number of parentheses we must add to make the
resulting string valid.

Example 1:

    Input: "())"
    Output: 1

Example 2:

    Input: "((("
    Output: 3

Example 3:

    Input: "()"
    Output: 0

Example 4:

    Input: "()))(("
    Output: 4

Note:

    S.length <= 1000
    S only consists of '(' and ')' characters.
*/

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut ans = 0;
        let mut left_cnt: u32 = 0;
        for c in s.chars() {
            match c {
                '(' => left_cnt += 1,
                ')' => {
                    left_cnt = left_cnt.checked_sub(1).unwrap_or_else(|| {
                        ans += 1;
                        0
                    })
                }
                _ => unreachable!(),
            }
        }
        ans + left_cnt as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(Solution::min_add_to_make_valid(String::from("())")), 1);
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::min_add_to_make_valid(String::from("(((")), 3);
    }

    #[test]
    fn test_case3() {
        assert_eq!(Solution::min_add_to_make_valid(String::from("()")), 0);
    }

    #[test]
    fn test_case4() {
        assert_eq!(Solution::min_add_to_make_valid(String::from("()))((")), 4);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::min_add_to_make_valid(String::from("()))((")));
    }
}
