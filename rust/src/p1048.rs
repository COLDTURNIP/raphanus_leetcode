/*
Problem 1048. Longest String Chain
==================================

https://leetcode.com/problems/longest-string-chain/

Given a list of words, each word consists of English lowercase letters.

Let's say word1 is a predecessor of word2 if and only if we can add exactly one letter anywhere in
word1 to make it equal to word2.  For example, "abc" is a predecessor of "abac".

A word chain is a sequence of words [word_1, word_2, ..., word_k] with k >= 1, where word_1 is a
predecessor of word_2, word_2 is a predecessor of word_3, and so on.

Return the longest possible length of a word chain with words chosen from the given list of words.

Example 1:

    Input: words = ["a","b","ba","bca","bda","bdca"]
    Output: 4
    Explanation: One of the longest word chain is "a","ba","bda","bdca".

Example 2:

    Input: words = ["xbc","pcxbcf","xb","cxbc","pcxbc"]
    Output: 5

Constraints:

    - 1 <= words.length <= 1000
    - 1 <= words[i].length <= 16
    - words[i] only consists of English lowercase letters.
*/

impl Solution {
    fn is_predecessor(w1: &str, w2: &str) -> bool {
        if w1.len() != w2.len() + 1 {
            return false;
        }
        let mut iter1 = w1.chars();
        let mut changed = false;
        for c2 in w2.chars() {
            let mut c1 = iter1.next();
            if c1 != Some(c2) && !changed {
                c1 = iter1.next();
                changed = true;
            }
            if c1 != Some(c2) {
                return false;
            }
        }
        true
    }

    pub fn longest_str_chain(mut words: Vec<String>) -> i32 {
        words.sort_unstable_by(|w1, w2| w1.len().cmp(&w2.len()).then(w1.cmp(w2)));
        let len = words.len();
        let mut cnt = vec![1_i32; len];
        for (i, word) in words.iter().enumerate().skip(1) {
            for (j, _) in words
                .iter()
                .enumerate()
                .take(i)
                .filter(|(_, p)| Solution::is_predecessor(word, p))
            {
                cnt[i] = cnt[i].max(cnt[j] + 1);
            }
        }
        cnt.into_iter().max().unwrap_or(0)
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
            Solution::longest_str_chain(vec![
                "a".to_owned(),
                "b".to_owned(),
                "ba".to_owned(),
                "bca".to_owned(),
                "bda".to_owned(),
                "bdca".to_owned()
            ]),
            4
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::longest_str_chain(vec![
                "xbc".to_owned(),
                "pcxbcf".to_owned(),
                "xb".to_owned(),
                "cxbc".to_owned(),
                "pcxbc".to_owned()
            ]),
            5
        );
    }

    #[test]
    fn test_no_common() {
        assert_eq!(
            Solution::longest_str_chain(vec![
                "xbc".to_owned(),
                "aaaa".to_owned(),
                "cdef".to_owned()
            ]),
            1
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(Solution::longest_str_chain(Vec::new()), 0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::longest_str_chain(vec![
                "xbc".to_owned(),
                "pcxbcf".to_owned(),
                "xb".to_owned(),
                "cxbc".to_owned(),
                "pcxbc".to_owned(),
            ])
        });
    }
}
