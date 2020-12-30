/*
Problem 316. Remove Duplicate Letters
=====================================

https://leetcode.com/problems/remove-duplicate-letters/

Given a string s, remove duplicate letters so that every letter appears once and only once. You
must make sure your result is the smallest in lexicographical order among all possible results.

Note: This question is the same as 1081:
https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/

Example 1:

    Input: s = "bcabc"
    Output: "abc"

Example 2:

    Input: s = "cbacdcbc"
    Output: "acdb"

Constraints:

    - 1 <= s.length <= 10^4
    - s consists of lowercase English letters.
*/

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut cnt = vec![0; 26];
        for bc in s.bytes() {
            cnt[(bc - b'a') as usize] += 1;
        }

        let mut inserted = vec![false; 26];
        let mut result: Vec<u8> = Vec::new();
        for bc in s.bytes() {
            cnt[(bc - b'a') as usize] -= 1;
            if inserted[(bc - b'a') as usize] {
                continue;
            }

            // pop the inserted characters which are greater then current one
            while let Some(&obc) = result.last() {
                if obc > bc && cnt[(obc - b'a') as usize] > 0 {
                    result.pop();
                    inserted[(obc - b'a') as usize] = false;
                } else {
                    break;
                }
            }

            // append the current charactor
            result.push(bc);
            inserted[(bc - b'a') as usize] = true;
        }
        std::str::from_utf8(&result).unwrap().to_string()
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
            Solution::remove_duplicate_letters("bcabc".to_owned()),
            "abc".to_owned()
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::remove_duplicate_letters("cbacdcbc".to_owned()),
            "acdb".to_owned()
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(
            Solution::remove_duplicate_letters(String::new()),
            String::new()
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::remove_duplicate_letters("cbacdcbc".to_owned()));
    }
}
