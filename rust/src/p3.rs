/*
Problem 3. Longest Substring Without Repeating Characters
=======================================

https://leetcode.com/problems/longest-substring-without-repeating-characters/

Given a string s, find the length of the longest substring without repeating characters.


Example 1:

    Input: s = "abcabcbb"
    Output: 3
    Explanation: The answer is "abc", with the length of 3.

Example 2:

    Input: s = "bbbbb"
    Output: 1
    Explanation: The answer is "b", with the length of 1.

Example 3:

    Input: s = "pwwkew"
    Output: 3
    Explanation: The answer is "wke", with the length of 3.
    Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.

Example 4:

    Input: s = ""
    Output: 0


Constraints:

    0 <= s.length <= 5 * 104
    s consists of English letters, digits, symbols and spaces.
*/
use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut window = HashSet::<char>::new();
        let mut ans = 0;
        let mut low = 0;
        for (i, cur) in s.iter().enumerate() {
            if !window.insert(*cur) {
                ans = ans.max(window.len());
                while s[low] != *cur {
                    window.remove(&s[low]);
                    low += 1;
                }
                if low < i {
                    low += 1;
                }
            }
        }
        ans.max(window.len()) as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_owned()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_owned()), 1);
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_owned()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("".to_owned()), 0);
        assert_eq!(Solution::length_of_longest_substring(" ".to_owned()), 1);
    }

    #[test]
    fn test_with_symbol() {
        assert_eq!(
            Solution::length_of_longest_substring("aabaab!bb".to_owned()),
            3
        );
    }

    #[test]
    fn test_dvdf() {
        assert_eq!(Solution::length_of_longest_substring("dvdf".to_owned()), 3);
    }

    #[test]
    fn test_ggububgvfk() {
        assert_eq!(
            Solution::length_of_longest_substring("ggububgvfk".to_owned()),
            6
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::length_of_longest_substring("abcabcbb".to_owned()))
    }
}
