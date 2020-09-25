/*
Problem 97. Interleaving String
===============================

Given s1, s2, and s3, find whether s3 is formed by the interleaving of s1 and s2.

Example 1:

    Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
    Output: true

Example 2:

    Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbbaccc"
    Output: false

Example 3:

    Input: s1 = "", s2 = "", s3 = ""
    Output: true

Constraints:

    0 <= s1.length, s2.length <= 100
    0 <= s3.length <= 200
    s1, s2, and s3 consist of lower-case English letters.
*/

use std::iter;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let len1 = s1.len();
        let len2 = s2.len();
        if len1 + len2 != s3.len() {
            return false;
        }
        let s1 = s1.chars().collect::<Vec<char>>();
        let s2 = s2.chars().collect::<Vec<char>>();

        // seen[take1][take2] means, given the number of chars taken from s1 & s2, whether it was
        // possible to fulfill all needs of s3[0 .. take1 + take2]
        let mut seen: Vec<Vec<bool>> =
            iter::repeat_with(|| iter::repeat(false).take(len2 + 1).collect())
                .take(len1 + 1)
                .collect();
        seen[0][0] = true;

        for (i3, c) in s3.chars().enumerate() {
            let mut possible = false;
            for take1 in (i3 + 1).saturating_sub(len2)..=(i3 + 1).min(len1) {
                let take2 = i3 + 1 - take1;
                let from1 = take1
                    .checked_sub(1)
                    .map(|i1| s1[i1] == c && seen[i1][take2])
                    .unwrap_or(false);
                let from2 = take2
                    .checked_sub(1)
                    .map(|i2| s2[i2] == c && seen[take1][i2])
                    .unwrap_or(false);
                seen[take1][take2] = from1 || from2;
                possible |= from1 || from2;
            }
            if !possible {
                // early stop
                return false;
            }
        }
        seen[len1][len2]
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    fn solution(s1: &str, s2: &str, s3: &str) -> bool {
        Solution::is_interleave(s1.to_string(), s2.to_string(), s3.to_string())
    }

    #[test]
    fn test_aabcc_dbbca_aabbcbcac() {
        assert!(solution("aabcc", "dbbca", "aadbbcbcac"));
    }

    #[test]
    fn test_aabcc_dbbca_aadbbbaccc() {
        assert!(!solution("aabcc", "dbbca", "aadbbbaccc"));
    }

    #[test]
    fn test_empty() {
        assert!(solution("", "", ""));
        assert!(!solution("1", "", ""));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::is_interleave(
                "aabcc".to_owned(),
                "dbbca".to_owned(),
                "aabbcbcac".to_owned(),
            )
        });
    }
}
