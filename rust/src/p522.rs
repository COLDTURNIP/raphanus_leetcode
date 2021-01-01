/*
Problem 522. Longest Uncommon Subsequence II
============================================

https://leetcode.com/problems/longest-uncommon-subsequence-ii/

Given a list of strings, you need to find the longest uncommon subsequence among them. The longest
uncommon subsequence is defined as the longest subsequence of one of these strings and this
subsequence should not be any subsequence of the other strings.

A subsequence is a sequence that can be derived from one sequence by deleting some characters
without changing the order of the remaining elements. Trivially, any string is a subsequence of
itself and an empty string is a subsequence of any string.

The input will be a list of strings, and the output needs to be the length of the longest uncommon
subsequence. If the longest uncommon subsequence doesn't exist, return -1.

Example 1:

    Input: "aba", "cdc", "eae"
    Output: 3

Note:

    - All the given strings' lengths will not exceed 10.
    - The length of the given list will be in the range of [2, 50].
*/

#[derive(PartialEq, Eq)]
enum CmpSeq {
    Uniq,
    Same,
    Sub,
}

impl Solution {
    fn cmp_subseq(sub: &str, orig: &str) -> CmpSeq {
        if sub.len() > orig.len() {
            CmpSeq::Uniq
        } else if sub.is_empty() {
            CmpSeq::Sub
        } else {
            let mut sub_iter = sub.chars().peekable();
            for c in orig.chars() {
                if Some(&c) == sub_iter.peek() {
                    sub_iter.next();
                }
            }
            match (sub_iter.peek().is_none(), orig.len() == sub.len()) {
                (true, true) => CmpSeq::Same,
                (true, false) => CmpSeq::Sub,
                (false, _) => CmpSeq::Uniq,
            }
        }
    }

    pub fn find_lu_slength(mut strs: Vec<String>) -> i32 {
        strs.sort_by_key(|s| -(s.len() as i32));
        let mut candidate: Vec<usize> = (0..strs.len()).collect();
        let mut next_candidate = Vec::new();
        while !candidate.is_empty() {
            let mut c_iter = candidate.iter();
            let cur_lus = &strs[*c_iter.next().unwrap()];
            let mut found = true;
            for si in c_iter {
                let cmp = Self::cmp_subseq(&strs[*si], cur_lus);
                found &= cmp != CmpSeq::Same;
                if cmp == CmpSeq::Uniq {
                    next_candidate.push(*si);
                }
            }
            if found {
                return cur_lus.len() as i32;
            }
            std::mem::swap(&mut candidate, &mut next_candidate);
            next_candidate.truncate(0);
        }
        -1
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
            Solution::find_lu_slength(
                ["aba", "cdc", "eae"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            ),
            3
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::find_lu_slength(["aaa", "aaa", "aa"].iter().map(|s| s.to_string()).collect()),
            -1
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::find_lu_slength(
                ["aabbcc", "aabbcc", "cb"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            ),
            2
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::find_lu_slength(
                ["aba", "cdc", "eae"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            )
        });
    }
}
