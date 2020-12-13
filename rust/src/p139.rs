/*
Problem 139. Word Break
=======================

https://leetcode.com/problems/word-break/

Given a non-empty string s and a dictionary wordDict containing a list of non-empty words,
determine if s can be segmented into a space-separated sequence of one or more dictionary words.

Note:

    - The same word in the dictionary may be reused multiple times in the segmentation.
    - You may assume the dictionary does not contain duplicate words.

Example 1:

    Input: s = "leetcode", wordDict = ["leet", "code"]
    Output: true
    Explanation: Return true because "leetcode" can be segmented as "leet code".

Example 2:

    Input: s = "applepenapple", wordDict = ["apple", "pen"]
    Output: true
    Explanation: Return true because "applepenapple" can be segmented as "apple pen apple".
                 Note that you are allowed to reuse a dictionary word.

Example 3:

    Input: s = "catsandog", wordDict = ["cats", "dog", "sand", "and", "cat"]
    Output: false
*/

impl Solution {
    fn word_break_impl(s: &str, word_dict: &[String], breakable: &mut [Option<bool>]) -> bool {
        if s.is_empty() {
            return true;
        } else if let Some(result) = breakable[s.len() - 1] {
            return result;
        }
        let result = word_dict
            .iter()
            .filter(|w| s.starts_with(w.as_str()))
            .any(|w| Self::word_break_impl(&s[w.len()..], word_dict, breakable));
        breakable[s.len() - 1] = Some(result);
        result
    }

    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        Self::word_break_impl(&s, &word_dict, &mut vec![None; s.len()])
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert!(Solution::word_break(
            "leetcode".to_owned(),
            vec!["leet", "code"]
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        ));
    }

    #[test]
    fn test_case2() {
        assert!(Solution::word_break(
            "applepenapple".to_owned(),
            vec!["apple", "pen"]
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        ));
    }

    #[test]
    fn test_case3() {
        assert!(Solution::word_break(
            "catsanddog".to_owned(),
            vec!["cats", "dog", "sand", "and", "cat"]
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        ));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::word_break(
                "catsanddog".to_owned(),
                vec!["cats", "dog", "sand", "and", "cat"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect(),
            )
        });
    }
}
