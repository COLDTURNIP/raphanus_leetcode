/*
Problem 763. Partition Labels
=======================================

https://leetcode.com/problems/partition-labels/

A string S of lowercase English letters is given. We want to partition this string into as many
parts as possible so that each letter appears in at most one part, and return a list of integers
representing the size of these parts.


Example 1:

    Input: S = "ababcbacadefegdehijhklij"
    Output: [9,7,8]

    Explanation:
    The partition is "ababcbaca", "defegde", "hijhklij".
    This is a partition so that each letter appears in at most one part.
    A partition like "ababcbacadefegde", "hijhklij" is incorrect, because it splits S into less parts.


Note:

    S will have length in range [1, 500].
    S will consist of lowercase English letters ('a' to 'z') only.
*/
use std::collections::BTreeMap;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
struct PosRec {
    start: usize,
    end: usize,
}

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut appear = BTreeMap::new();
        for (i, c) in s.chars().enumerate() {
            (*appear.entry(c).or_insert(PosRec {
                start: i,
                end: i + 1,
            }))
            .end = i + 1;
        }
        let mut pos_rec = appear.into_iter().map(|(_, v)| v).collect::<Vec<PosRec>>();
        pos_rec.sort_unstable();
        let mut ans = Vec::new();
        let mut cur_start = 0;
        let mut cur_end = 0;
        let mut push_result = |start, end| {
            let len = end - start;
            if len > 0 {
                ans.push(len as i32);
            }
        };
        for pos in pos_rec {
            if cur_end == pos.start {
                push_result(cur_start, cur_end);
                cur_start = pos.start;
                cur_end = pos.end;
            } else if cur_end < pos.end {
                cur_end = pos.end;
            }
        }
        push_result(cur_start, cur_end);
        ans
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
            Solution::partition_labels("ababcbacadefegdehijhklij".to_owned()),
            vec![9, 7, 8]
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::partition_labels("ababcbacadefegdehijhklij".to_owned()))
    }
}
