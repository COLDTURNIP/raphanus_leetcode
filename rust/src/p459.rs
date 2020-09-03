/*
459. Repeated Substring Pattern
===============================

https://leetcode.com/problems/repeated-substring-pattern/

Given a non-empty string check if it can be constructed by taking a substring of it and appending
multiple copies of the substring together. You may assume the given string consists of lowercase
English letters only and its length will not exceed 10000.

 

Example 1:

    Input: "abab"
    Output: True
    Explanation: It's the substring "ab" twice.

Example 2:

    Input: "aba"
    Output: False

Example 3:

    Input: "abcabcabcabc"
    Output: True
    Explanation: It's the substring "abc" four times. (And the substring "abcabc" twice.)
*/

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        let len = s.len();
        for sub_len in (1..=len/2).filter(|n| len % n == 0) {
            let sub = &s[..sub_len];
            if (1..len/sub_len).all(|i| {
                sub == &s[i*sub_len..i*sub_len+sub_len]
            }) {
                return true;
            }
        }
        false
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_abab() {
        assert_eq!(Solution::repeated_substring_pattern("abab".to_owned()), true);
    }

    #[test]
    fn test_aba() {
        assert_eq!(Solution::repeated_substring_pattern("aba".to_owned()), false);
    }

    #[test]
    fn test_abc_5times() {
        assert_eq!(Solution::repeated_substring_pattern("abcabcabcabcabc".to_owned()), true);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::repeated_substring_pattern("abcabcabcabcabc".to_owned()));
    }
}
