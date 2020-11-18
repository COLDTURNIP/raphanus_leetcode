/*
Problem 647. Palindromic Substrings
===================================

https://leetcode.com/problems/palindromic-substrings/

Given a string, your task is to count how many palindromic substrings in this string.

The substrings with different start indexes or end indexes are counted as different substrings even
they consist of same characters.

Example 1:

    Input: "abc"
    Output: 3
    Explanation: Three palindromic strings: "a", "b", "c".

Example 2:

    Input: "aaa"
    Output: 6
    Explanation: Six palindromic strings: "a", "a", "a", "aa", "aa", "aaa".

Note:

    - The input string length won't exceed 1000.
*/

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let len = s.len();
        let mut cnt = 0;
        for i in 0..len {
            cnt += 1;

            for d in 1..=i {
                if i + d >= len || s[i - d] != s[i + d] {
                    break;
                } else {
                    cnt += 1;
                }
            }

            for d in 0..=i {
                if i + 1 + d >= len || s[i - d] != s[i + 1 + d] {
                    break;
                } else {
                    cnt += 1;
                }
            }
        }
        cnt
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_abc() {
        assert_eq!(Solution::count_substrings("abc".to_owned()), 3);
    }

    #[test]
    fn test_aaa() {
        assert_eq!(Solution::count_substrings("aaa".to_owned()), 6);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::count_substrings("alkjqworiuojljlwer2342342789987764a".to_owned()));
    }
}
