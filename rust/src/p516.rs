/*
Problem 516. Longest Palindromic Subsequence
============================================

https://leetcode.com/problems/longest-palindromic-subsequence/

Given a string s, find the longest palindromic subsequence's length in s. You may assume that the
maximum length of s is 1000.

Example 1:
    Input: "bbbab"
    Output: 4
    One possible longest palindromic subsequence is "bbbb".

Example 2:
    Input: "cbbd"
    Output: 2
    One possible longest palindromic subsequence is "bb".

Constraints:

    - 1 <= s.length <= 1000
    - s consists only of lowercase English letters.
*/

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let s: Vec<char> = s.chars().collect();
        let mut max_len = vec![0; s.len()];
        let mut prev_max_len = vec![0; s.len()];

        // At the jth iteration:
        //   - max_len[i] means the max palindrome length in substring s[i:=j]
        //   - prev_max_len[i] means the max palindrome length in substring s[i:=j-1]
        for (j, &cj) in s.iter().enumerate() {
            max_len[j] = 1;
            for i in (0..j).rev() {
                max_len[i] = if s[i] == cj {
                    prev_max_len[i + 1] + 2
                } else {
                    prev_max_len[i].max(max_len[i + 1])
                };
            }
            std::mem::swap(&mut max_len, &mut prev_max_len);
        }
        prev_max_len[0]
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_bbbab() {
        assert_eq!(Solution::longest_palindrome_subseq("bbbab".to_owned()), 4);
    }

    #[test]
    fn test_cbbd() {
        assert_eq!(Solution::longest_palindrome_subseq("cbbd".to_owned()), 2);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::longest_palindrome_subseq(String::new()), 0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::longest_palindrome_subseq("bbbab".to_owned()));
    }
}
