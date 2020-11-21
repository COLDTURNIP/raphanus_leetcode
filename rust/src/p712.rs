/*
Problem 712. Minimum ASCII Delete Sum for Two Strings
=====================================================

https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/

Given two strings s1, s2, find the lowest ASCII sum of deleted characters to make two strings
equal.

Example 1:

    Input: s1 = "sea", s2 = "eat"
    Output: 231
    Explanation: Deleting "s" from "sea" adds the ASCII value of "s" (115) to the sum.
    Deleting "t" from "eat" adds 116 to the sum.
    At the end, both strings are equal, and 115 + 116 = 231 is the minimum sum possible to achieve this.

Example 2:

    Input: s1 = "delete", s2 = "leet"
    Output: 403
    Explanation: Deleting "dee" from "delete" to turn the string into "let",
    adds 100[d]+101[e]+101[e] to the sum.  Deleting "e" from "leet" adds 101[e] to the sum.
    At the end, both strings are equal to "let", and the answer is 100+101+101+101 = 403.
    If instead we turned both strings into "lee" or "eet", we would get answers of 433 or 417, which are higher.

Note:

    - 0 < s1.length, s2.length <= 1000.
    - All elements of each string will have an ASCII value in [97, 122].
*/

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let s1: Vec<i32> = s1.bytes().map(|b| b as i32).collect();
        let s2: Vec<i32> = s2.bytes().map(|b| b as i32).collect();

        if s1.is_empty() {
            return s2.into_iter().sum();
        } else if s2.is_empty() {
            return s1.into_iter().sum();
        }

        // rm_sum[i][j] means the removed char sum to make sum(s1[:i]) equals to sum(s2[:j])
        let mut rm_sum = vec![vec![0; s2.len() + 1]; s1.len() + 1];

        // if s2 is empty, s1 need to remove all the characters
        for (i, &n) in s1.iter().enumerate() {
            rm_sum[i + 1][0] = rm_sum[i][0] + n;
        }

        // if s1 is empty, s2 need to remove all the characters
        for (j, &n) in s2.iter().enumerate() {
            rm_sum[0][j + 1] = rm_sum[0][j] + n;
        }

        for (i, &n1) in s1.iter().enumerate() {
            for (j, &n2) in s2.iter().enumerate() {
                rm_sum[i + 1][j + 1] = if n1 == n2 {
                    // no char need to be removed
                    rm_sum[i][j]
                } else {
                    // minimize the sum between the paths of s1 or s2
                    (rm_sum[i][j + 1] + n1).min(rm_sum[i + 1][j] + n2)
                };
            }
        }

        rm_sum[s1.len()][s2.len()]
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_min_del_sum1() {
        assert_eq!(
            Solution::minimum_delete_sum("sea".to_owned(), "eat".to_owned()),
            231
        );
    }

    #[test]
    fn test_min_del_sum2() {
        assert_eq!(
            Solution::minimum_delete_sum("delete".to_owned(), "leet".to_owned()),
            403
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::minimum_delete_sum("sea".to_owned(), "eat".to_owned()));
    }
}
