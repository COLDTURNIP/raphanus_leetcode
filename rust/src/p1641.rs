/*
Problem 1641. Count Sorted Vowel Strings
========================================

https://leetcode.com/problems/count-sorted-vowel-strings/

Given an integer n, return the number of strings of length n that consist only of vowels (a, e, i,
o, u) and are lexicographically sorted.

A string s is lexicographically sorted if for all valid i, s[i] is the same as or comes before
s[i+1] in the alphabet.

Example 1:

    Input: n = 1
    Output: 5
    Explanation: The 5 sorted strings that consist of vowels only are ["a","e","i","o","u"].

Example 2:

    Input: n = 2
    Output: 15
    Explanation: The 15 sorted strings that consist of vowels only are
    ["aa","ae","ai","ao","au","ee","ei","eo","eu","ii","io","iu","oo","ou","uu"].
    Note that "ea" is not a valid string since 'e' comes after 'a' in the alphabet.

Example 3:

    Input: n = 33
    Output: 66045

Constraints:

    - 1 <= n <= 50
*/

impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        if n < 1 {
            0
        } else {
            let mut cnt = vec![1; 5];
            for _ in 1..n {
                for i in 1..5 {
                    cnt[i] += cnt[i - 1];
                }
            }
            cnt.into_iter().sum()
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::count_vowel_strings(1), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::count_vowel_strings(2), 15);
    }

    #[test]
    fn test_33() {
        assert_eq!(Solution::count_vowel_strings(33), 66045);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::count_vowel_strings(33));
    }
}
