/*
Problem 76. Minimum Window Substring
====================================

https://leetcode.com/problems/minimum-window-substring/

Given a string S and a string T, find the minimum window in S which will contain all the characters
in T in complexity O(n).

Example:

    Input: S = "ADOBECODEBANC", T = "ABC"
    Output: "BANC"

Note:

    - If there is no such window in S that covers all characters in T, return the empty string "".
    - If there is such window, you are guaranteed that there will always be only one unique minimum
      window in S.
*/

use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.is_empty() || t.is_empty() {
            return String::new();
        }
        // Note: if it needs to maximize the performance, change the expect_cnt to counter array of
        // all possible character set.
        let mut match_addr = VecDeque::<(char, usize)>::new();
        let mut match_cnt = 0;
        let mut expect_cnt = t.chars().fold(HashMap::new(), |mut cnt, c| {
            *cnt.entry(c).or_insert(0) += 1;
            cnt
        });
        let expect_match_cnt = expect_cnt.len();
        let mut ans = None;
        for (i, c) in s.chars().enumerate() {
            let exp_cnt = if let Some(ref_n) = expect_cnt.get_mut(&c) {
                ref_n
            } else {
                continue;
            };
            match_addr.push_back((c, i));
            *exp_cnt -= 1;
            if *exp_cnt == 0 {
                match_cnt += 1;
            }
            while match_cnt == expect_match_cnt {
                let (rm_c, rm_i) = match_addr.pop_front().unwrap();
                let new_rng = rm_i..i + 1;
                ans = match ans {
                    None => Some(new_rng),
                    Some(rng) if rng.len() > i + 1 - rm_i => Some(new_rng),
                    _ => ans,
                };
                if let Some(ref_n) = expect_cnt.get_mut(&rm_c) {
                    if *ref_n == 0 {
                        match_cnt -= 1;
                    }
                    *ref_n += 1;
                };
            }
        }
        ans.and_then(|rng| s.get(rng)).map(|s| s.to_string()).unwrap_or_else(String::new)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_abc() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_owned(), "ABC".to_owned()),
            "BANC".to_owned()
        );
    }

    #[test]
    fn test_aa_from_a() {
        assert_eq!(
            Solution::min_window("a".to_owned(), "aa".to_owned()),
            "".to_owned()
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::min_window("ADOBECODEBANC".to_owned(), "ABC".to_owned());
        });
    }
}
