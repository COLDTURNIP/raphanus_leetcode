/*
Problem 131. Palindrome Partitioning
====================================

https://leetcode.com/problems/palindrome-partitioning/

Given a string s, partition s such that every substring of the partition is a palindrome. Return
all possible palindrome partitioning of s.

A palindrome string is a string that reads the same backward as forward.

Example 1:

    Input: s = "aab"
    Output: [["a","a","b"],["aa","b"]]
    Example 2:

    Input: s = "a"
    Output: [["a"]]

Constraints:

    - 1 <= s.length <= 16
    - s contains only lowercase English letters.
*/

impl Solution {
    fn is_palindrome(s: &str) -> bool {
        let bytes = s.as_bytes();
        let len = bytes.len();
        if len <= 1 {
            return true;
        }
        let (mut li, mut ri) = (0, len - 1);
        while li < ri {
            if bytes[li] != bytes[ri] {
                return false;
            }
            li += 1;
            ri -= 1;
        }
        true
    }

    fn partition_impl(parts: &mut Vec<String>, rest: &str, ans: &mut Vec<Vec<String>>) {
        if rest.is_empty() {
            ans.push(parts.clone());
            return;
        }
        for head_len in 1..=rest.len() {
            let (head, tail) = rest.split_at(head_len);
            if Self::is_palindrome(head) {
                parts.push(head.to_string());
                Self::partition_impl(parts, tail, ans);
                parts.pop();
            }
        }
    }

    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut ans = Vec::new();
        Self::partition_impl(&mut Vec::new(), &s, &mut ans);
        ans
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;
    use std::collections::HashSet;

    fn check(input: &str, expect: Vec<Vec<&str>>) {
        let expect: HashSet<Vec<String>> = expect
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            })
            .collect();
        let exact_vec = Solution::partition(input.to_string());
        let exact: HashSet<Vec<String>> = expect.iter().cloned().collect();
        assert_eq!(exact_vec.len(), expect.len());
        assert_eq!(expect, exact);
    }

    #[test]
    fn test_case1() {
        check("aab", vec![vec!["a", "a", "b"], vec!["aa", "b"]]);
    }

    #[test]
    fn test_case2() {
        check("a", vec![vec!["a"]]);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::partition("aabccdasdbbsfa".to_owned()));
    }
}
