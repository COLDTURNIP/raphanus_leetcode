/*
Problem 44. Wildcard Matching
=============================

https://leetcode.com/problems/wildcard-matching/

Given an input string (s) and a pattern (p), implement wildcard pattern matching with support for
'?' and '*'.

    - '?' Matches any single character.
    - '*' Matches any sequence of characters (including the empty sequence).
    - The matching should cover the entire input string (not partial).

Note:

    - s could be empty and contains only lowercase letters a-z.
    - p could be empty and contains only lowercase letters a-z, and characters like ? or *.

Example 1:

    Input:
        s = "aa"
        p = "a"
    Output: false
    Explanation: "a" does not match the entire string "aa".

Example 2:

    Input:
        s = "aa"
        p = "*"
    Output: true
    Explanation: '*' matches any sequence.

Example 3:

    Input:
        s = "cb"
        p = "?a"
    Output: false
    Explanation: '?' matches 'c', but the second letter is 'a', which does not match 'b'.

Example 4:

    Input:
        s = "adceb"
        p = "*a*b"
    Output: true
    Explanation: The first '*' matches the empty sequence, while the second '*' matches the substring "dce".

Example 5:

    Input:
        s = "acdcb"
        p = "a*c?b"
    Output: false
*/

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if p.is_empty() {
            return s.is_empty();
        }
        // TODO: this solution is DP. Change to 2-cursor scheme to imporove performance.
        let s = s.chars().collect::<Vec<char>>();
        let p = p.chars().collect::<Vec<char>>();
        let mut buf_old = vec![false; p.len() + 1];
        let mut buf_new = vec![false; p.len() + 1];
        buf_old[0] = true;
        for (i, c) in p.iter().enumerate() {
            buf_old[i + 1] = *c == '*' && buf_old[i];
        }
        for c in s.into_iter() {
            buf_new[0] = false;
            for (pi, pc) in p.iter().enumerate() {
                match *pc {
                    '*' => buf_new[pi + 1] = buf_old[pi + 1] || buf_new[pi],
                    '?' => buf_new[pi + 1] = buf_old[pi],
                    pc => buf_new[pi + 1] = pc == c && buf_old[pi],
                }
            }
            std::mem::swap(&mut buf_old, &mut buf_new);
        }
        buf_old.last() == Some(&true)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    #[test]
    fn test_always_match() {
        assert!(Solution::is_match("aa".to_owned(), "*".to_owned()));
    }

    #[test]
    fn test_not_match() {
        assert!(!Solution::is_match("cb".to_owned(), "?a".to_owned()));
    }

    #[test]
    fn test_not_match_exact() {
        assert!(!Solution::is_match("aa".to_owned(), "a".to_owned()));
    }

    #[test]
    fn test_not_match_exceed() {
        assert!(!Solution::is_match("a".to_owned(), "aa".to_owned()));
    }

    #[test]
    fn test_not_match_wildcard() {
        assert!(!Solution::is_match("aab".to_owned(), "c*a*b".to_owned()));
    }

    #[test]
    fn test_empty() {
        assert!(!Solution::is_match("".to_owned(), "a".to_owned()));
        assert!(!Solution::is_match("aa".to_owned(), "".to_owned()));
        assert!(Solution::is_match("".to_owned(), "".to_owned()));
        assert!(Solution::is_match("".to_owned(), "*".to_owned()));
    }

    #[test]
    fn test_wildcard_interleave() {
        assert!(Solution::is_match("adceb".to_owned(), "*a*b".to_owned()));
    }

    #[test]
    fn test_wildcard_interleave1() {
        assert!(Solution::is_match("adceb".to_owned(), "*a*b".to_owned()));
    }

    #[test]
    fn test_wildcard_interleave2() {
        assert!(Solution::is_match("adceb".to_owned(), "a*c?b".to_owned()));
    }

    #[test]
    fn test_wildcard_multiple() {
        assert!(!Solution::is_match(
            "bbbaab".to_string(),
            "a**?***".to_string()
        ));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::is_match("abwroijkljsewrovjln".to_owned(), "j?wr?vj".to_owned()));
    }
}
