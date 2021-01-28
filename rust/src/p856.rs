/*
Problem 856. Score of Parentheses
=================================

https://leetcode.com/problems/score-of-parentheses/

Given a balanced parentheses string S, compute the score of the string based on the following rule:

    - () has score 1
    - AB has score A + B, where A and B are balanced parentheses strings.
    - (A) has score 2 * A, where A is a balanced parentheses string.

Example 1:

    Input: "()"
    Output: 1

Example 2:

    Input: "(())"
    Output: 2

Example 3:

    Input: "()()"
    Output: 2

Example 4:

    Input: "(()(()))"
    Output: 6

Note:

    - S is a balanced parentheses string, containing only ( and ).
    - 2 <= S.length <= 50
*/

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut score = Vec::with_capacity(s.len() / 2);
        let mut cur_score = 0;
        for c in s.chars() {
            match c {
                '(' => {
                    score.push(cur_score);
                    cur_score = 0;
                }
                ')' => {
                    if let Some(last_score) = score.pop() {
                        // std::mem::swap(&mut cur_score, &mut last_score);
                        // cur_score += 2 * last_score.max(1);
                        cur_score = last_score + 2 * cur_score.max(1);
                    }
                }
                _ => unreachable!(),
            }
        }
        cur_score / 2 // the most outer result does not apply doulbe score
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(Solution::score_of_parentheses(String::from("()")), 1);
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::score_of_parentheses(String::from("(())")), 2);
    }

    #[test]
    fn test_case3() {
        assert_eq!(Solution::score_of_parentheses(String::from("()()")), 1 + 1);
    }

    #[test]
    fn test_case4() {
        assert_eq!(
            Solution::score_of_parentheses(String::from("(()(()))")),
            2 * (1 + 2)
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::score_of_parentheses(String::new()), 0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::score_of_parentheses(String::from("(()(((()))()(()())((((())))()()))(()))"))
        });
    }
}
