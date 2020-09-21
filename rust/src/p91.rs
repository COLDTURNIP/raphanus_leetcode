/*
Problem 91. Decode Ways
=======================

https://leetcode.com/problems/decode-ways/

A message containing letters from A-Z is being encoded to numbers using the following mapping:

    'A' -> 1
    'B' -> 2
    ...
    'Z' -> 26

Given a non-empty string containing only digits, determine the total number of ways to decode it.

Example 1:

    Input: "12"
    Output: 2
    Explanation: It could be decoded as "AB" (1 2) or "L" (12).

Example 2:

    Input: "226"
    Output: 3
    Explanation: It could be decoded as "BZ" (2 26), "VF" (22 6), or "BBF" (2 2 6).
*/

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut prev1_cnt = 1; // count of paths getting the last char
        let mut prev2_cnt = 1; // count of paths getting the last 2 char
        let mut last = '0';
        for cur in s.chars() {
            if cur == '0' {
                if last == '0' {
                    return 0;
                }
                prev1_cnt = 0;
            }
            match cur {
                '0'..='6' if last == '1' || last == '2' => {
                    let new_prev1 = prev2_cnt + prev1_cnt;
                    prev2_cnt = prev1_cnt;
                    prev1_cnt = new_prev1;
                }
                '7'..='9' if last == '1' => {
                    let new_prev1 = prev2_cnt + prev1_cnt;
                    prev2_cnt = prev1_cnt;
                    prev1_cnt = new_prev1;
                }
                _ => {
                    prev2_cnt = prev1_cnt;
                }
            }
            last = cur;
        }
        prev1_cnt
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    #[test]
    fn test_12() {
        assert_eq!(Solution::num_decodings("12".to_owned()), 2);
    }

    #[test]
    fn test_226() {
        assert_eq!(Solution::num_decodings("226".to_owned()), 3);
    }

    #[test]
    fn test_110() {
        assert_eq!(Solution::num_decodings("110".to_owned()), 1);
    }

    #[test]
    fn test_impossible() {
        assert_eq!(Solution::num_decodings("0101".to_owned()), 0);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::num_decodings(String::new()), 1);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::num_decodings("337921026321".to_owned()));
    }
}
