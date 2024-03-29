/*
Problem 392. Is Subsequence
===========================

https://leetcode.com/problems/is-subsequence/

Given a string s and a string t, check if s is subsequence of t.

A subsequence of a string is a new string which is formed from the original string by deleting some
(can be none) of the characters without disturbing the relative positions of the remaining
characters. (ie, "ace" is a subsequence of "abcde" while "aec" is not).

Follow up:

    If there are lots of incoming S, say S1, S2, ... , Sk where k >= 1B, and you want to check one
    by one to see if T has its subsequence. In this scenario, how would you change your code?

Credits:

    Special thanks to @pbrother for adding this problem and creating all test cases.

Example 1:

    Input: s = "abc", t = "ahbgdc"
    Output: true

Example 2:

    Input: s = "axc", t = "ahbgdc"
    Output: false

Constraints:

    0 <= s.length <= 100
    0 <= t.length <= 10^4
    Both strings consists only of lowercase characters.
*/

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut target_iter = s.chars().peekable();
        for c in t.chars() {
            match target_iter.peek() {
                None => break,
                Some(&tc) if tc == c => {
                    target_iter.next();
                }
                _ => {}
            }
        }
        target_iter.peek().is_none()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_found() {
        assert!(Solution::is_subsequence(
            "abc".to_owned(),
            "ahbgdc".to_owned()
        ));
    }

    #[test]
    fn test_not_found() {
        assert!(!Solution::is_subsequence(
            "axc".to_owned(),
            "ahbgdc".to_owned()
        ));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::is_subsequence("abc".to_owned(), "ahbgdc".to_owned()));
    }
}
