/*
Problem 32. Longest Valid Parentheses
=====================================

https://leetcode.com/problems/longest-valid-parentheses/

Given a string containing just the characters '(' and ')', find the length of the longest valid
(well-formed) parentheses substring.

Example 1:

    Input: "(()"
    Output: 2
    Explanation: The longest valid parentheses substring is "()"

Example 2:

    Input: ")()())"
    Output: 4
    Explanation: The longest valid parentheses substring is "()()"
*/

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut ans = 0;
        let mut buf = Vec::<usize>::new();
        let mut cur_start: usize = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                buf.push(i);
            } else {
                match buf.pop() {
                    Some(_) => {
                        ans = buf
                            .last()
                            .map(|&start| ans.max(i - start))
                            .unwrap_or_else(|| ans.max(i - cur_start + 1))
                    }
                    None => cur_start = i + 1,
                }
            }
        }
        ans as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    #[test]
    fn test_balanced() {
        assert_eq!(
            Solution::longest_valid_parentheses("()()()()()()".to_owned()),
            12
        );
    }

    #[test]
    fn test_inbalanced() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_owned()), 2);
    }

    #[test]
    fn test_inbalanced_not_continued() {
        assert_eq!(Solution::longest_valid_parentheses("()(()".to_owned()), 2);
    }

    #[test]
    fn test_start_opened() {
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_owned()), 4);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::longest_valid_parentheses("".to_owned()), 0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::longest_valid_parentheses("())))()()(((()()()())((".to_owned()));
    }
}
