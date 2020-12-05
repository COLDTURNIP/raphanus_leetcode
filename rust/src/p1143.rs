/*
Problem 1143. Longest Common Subsequence
========================================

https://leetcode.com/problems/longest-common-subsequence/

Given two strings text1 and text2, return the length of their longest common subsequence.

A subsequence of a string is a new string generated from the original string with some
characters(can be none) deleted without changing the relative order of the remaining characters.
(eg, "ace" is a subsequence of "abcde" while "aec" is not). A common subsequence of two strings is
a subsequence that is common to both strings.

If there is no common subsequence, return 0.

Example 1:

    Input: text1 = "abcde", text2 = "ace"
    Output: 3
    Explanation: The longest common subsequence is "ace" and its length is 3.

Example 2:

    Input: text1 = "abc", text2 = "abc"
    Output: 3
    Explanation: The longest common subsequence is "abc" and its length is 3.

Example 3:

    Input: text1 = "abc", text2 = "def"
    Output: 0
    Explanation: There is no such common subsequence, so the result is 0.

Constraints:

    - 1 <= text1.length <= 1000
    - 1 <= text2.length <= 1000
    - The input strings consist of lowercase English characters only.
*/

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1: Vec<char> = text1.chars().collect();
        let text2: Vec<char> = text2.chars().collect();
        let len = text1.len();

        // match_state[i] at iteration j: while matching text2[j], the longest matched length for
        // substring text1[0..=i]
        let mut match_state = vec![0; len];
        let mut last_state = vec![0; len];
        for c2 in text2 {
            std::mem::swap(&mut match_state, &mut last_state);
            for i in 0..len {
                match_state[i] = if text1[i] == c2 {
                    i.checked_sub(1).map(|i| last_state[i]).unwrap_or(0) + 1
                } else {
                    last_state[i].max(i.checked_sub(1).map(|i| match_state[i]).unwrap_or(0))
                }
            }
        }
        match_state.into_iter().last().unwrap_or(0)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::longest_common_subsequence("abcde".to_owned(), "ace".to_owned()),
            3
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::longest_common_subsequence("abc".to_owned(), "abc".to_owned()),
            3
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::longest_common_subsequence(
                "hofubmnylkra".to_owned(),
                "pqhgxgdofcvmr".to_owned()
            ),
            5 // hofmr
        );
    }

    #[test]
    fn test_case4() {
        assert_eq!(
            Solution::longest_common_subsequence("bsbininm".to_owned(), "jmjkbkjkv".to_owned()),
            1 // b or m
        );
    }

    #[test]
    fn test_no_common() {
        assert_eq!(
            Solution::longest_common_subsequence("abc".to_owned(), "def".to_owned()),
            0
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            Solution::longest_common_subsequence(String::new(), "abc".to_owned()),
            0
        );
        assert_eq!(
            Solution::longest_common_subsequence("abc".to_owned(), String::new()),
            0
        );
        assert_eq!(
            Solution::longest_common_subsequence(String::new(), String::new()),
            0
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::longest_common_subsequence("abcdefghijklmnop".to_owned(), "bjkop".to_owned())
        });
    }
}
