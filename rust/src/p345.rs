/*
Problem 345. Reverse Vowels of a String
=======================================

https://leetcode.com/problems/reverse-vowels-of-a-string/

Write a function that takes a string as input and reverse only the vowels of a string.

Example 1:

    Input: "hello"
    Output: "holle"

Example 2:

    Input: "leetcode"
    Output: "leotcede"
    Note:
    The vowels does not include the letter "y".
*/

impl Solution {
    fn is_vowel(ch: u8) -> bool {
        match ch {
            b'a' | b'e' | b'i' | b'o' | b'u' => true,
            b'A' | b'E' | b'I' | b'O' | b'U' => true,
            _ => false,
        }
    }

    pub fn reverse_vowels(s: String) -> String {
        if s.is_empty() {
            return s;
        }
        let mut s = s;
        unsafe {
            let sv = s.as_mut_vec();
            let (mut li, mut ri) = (0, sv.len() - 1);
            loop {
                while li < ri && !Self::is_vowel(sv[li]) {
                    li += 1;
                }
                while li < ri && !Self::is_vowel(sv[ri]) {
                    ri -= 1;
                }
                if li < ri {
                    sv.swap(li, ri);
                    li += 1;
                    ri -= 1;
                } else {
                    break s;
                }
            }
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_hello() {
        assert_eq!(
            Solution::reverse_vowels("hello".to_owned()),
            "holle".to_owned()
        );
    }

    #[test]
    fn test_leetcode() {
        assert_eq!(
            Solution::reverse_vowels("leetcode".to_owned()),
            "leotcede".to_owned()
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::reverse_vowels(String::new()), String::new());
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::reverse_vowels("a".to_owned()), "a".to_owned());
    }

    #[test]
    fn test_no_voles() {
        assert_eq!(Solution::reverse_vowels("b".to_owned()), "b".to_owned());
        assert_eq!(Solution::reverse_vowels("bc".to_owned()), "bc".to_owned());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::reverse_vowels("alskfj;lasjdfoeiwrjlk".to_owned()));
    }
}
