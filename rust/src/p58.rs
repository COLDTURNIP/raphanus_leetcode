/*
Problem 58. Length of Last Word
===============================

https://leetcode.com/problems/length-of-last-word/

Given a string s consists of upper/lower-case alphabets and empty space characters ' ', return the
length of last word (last word means the last appearing word if we loop from left to right) in the
string.

If the last word does not exist, return 0.

Note: A word is defined as a maximal substring consisting of non-space characters only.

Example:

    Input: "Hello World"
    Output: 5
*/

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut ret = None;
        for c in s.chars().rev() {
            match (c, ret.is_some()) {
                (' ', false) => {}
                (' ', true) => break,
                (_, true) => {
                    ret.as_mut().map(|n| *n += 1);
                }
                (_, false) => ret = Some(1),
            }
        }
        ret.unwrap_or(0)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_last_word() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_owned()), 5);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::length_of_last_word("           ".to_owned()), 0);
        assert_eq!(Solution::length_of_last_word(String::new()), 0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::length_of_last_word("Hello World".to_owned()));
    }
}
