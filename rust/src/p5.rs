/*
Problem 5. Longest Palindromic Substring
========================================

https://leetcode.com/problems/longest-palindromic-substring/

Given a string s, find the longest palindromic substring in s. You may assume that the maximum
length of s is 1000.

Example 1:

    Input: "babad"
    Output: "bab"
    Note: "aba" is also a valid answer.

Example 2:

    Input: "cbbd"
    Output: "bb"
*/

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return s;
        }
        let char_list = s.chars().collect::<Vec<char>>();
        let mut ans = (1, 0, 1);
        for i in 0..s.len() {
            let ppos = Self::check_palindrome_odd(&char_list, i)
                .max(Self::check_palindrome_even(&char_list, i));
            ans = ans.max(ppos)
        }
        char_list[ans.1..ans.2].iter().collect()
    }

    fn check_palindrome_odd(s: &[char], i: usize) -> (usize, usize, usize) {
        let len = s.len();
        (1..=i)
            .take_while(|&d| i + d < len && s[i - d] == s[i + d])
            .max()
            .map_or((1, i, i + 1), |d| (d * 2 + 1, i - d, i + d + 1))
    }

    fn check_palindrome_even(s: &[char], i: usize) -> (usize, usize, usize) {
        let len = s.len();
        (0..=i)
            .take_while(|&d| i + d + 1 < len && s[i - d] == s[i + 1 + d])
            .max()
            .map_or((1, i, i + 1), |d| (d * 2 + 2, i - d, i + d + 2))
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    #[test]
    fn test_babad() {
        assert_eq!(
            Solution::longest_palindrome("babad".to_owned()),
            "aba".to_owned()
        );
    }

    #[test]
    fn test_cbbd() {
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_owned()),
            "bb".to_owned()
        );
    }

    #[test]
    fn test_long() {
        assert_eq!(
            Solution::longest_palindrome("asfoiwerncabbac9915351aslkfj".to_owned()),
            "cabbac".to_owned()
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::longest_palindrome("".to_owned()), "".to_owned());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::longest_palindrome("asfoiwerncabbac9915351aslkfj".to_owned()));
    }
}
