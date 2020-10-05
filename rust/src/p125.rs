/*
Problem 125. Valid Palindrome
=============================

https://leetcode.com/problems/valid-palindrome/

Given a string, determine if it is a palindrome, considering only alphanumeric characters and
ignoring cases.

Note: For the purpose of this problem, we define empty string as valid palindrome.

Example 1:

Input: "A man, a plan, a canal: Panama"
Output: true
Example 2:

Input: "race a car"
Output: false


Constraints:

s consists only of printable ASCII characters.
*/

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<(usize, char)> = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .enumerate()
            .collect();
        for ((i1, c1), (i2, c2)) in chars.iter().zip(chars.iter().rev()) {
            if i1 > i2 {
                break;
            }
            if c1 != c2 {
                return false;
            }
        }
        true
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_true() {
        assert!(Solution::is_palindrome(
            "A man, a plan, a canal: Panama".to_owned()
        ));
    }

    #[test]
    fn test_false() {
        assert!(!Solution::is_palindrome("race a car".to_owned()));
        assert!(!Solution::is_palindrome("0P".to_owned()));
    }

    #[test]
    fn test_empty() {
        assert!(Solution::is_palindrome("".to_owned()));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_owned());
        });
    }
}
