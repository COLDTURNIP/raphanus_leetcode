/*
Problem 28. Implement strStr()
==============================

https://leetcode.com/problems/implement-strstr/

Implement strStr().

Return the index of the first occurrence of needle in haystack, or -1 if needle is not part of
haystack.

Example 1:

    Input: haystack = "hello", needle = "ll"
    Output: 2

Example 2:

    Input: haystack = "aaaaa", needle = "bba"
    Output: -1

Clarification:

    What should we return when needle is an empty string? This is a great question to ask during an
    interview.

    For the purpose of this problem, we will return 0 when needle is an empty string. This is
    consistent to C's strstr() and Java's indexOf().


Constraints:

    haystack and needle consist only of lowercase English characters.
*/

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map(|pos| pos as i32).unwrap_or(-1)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    #[test]
    fn test_hello() {
        assert_eq!(Solution::str_str("hello".to_owned(), "ll".to_owned()), 2);
    }

    #[test]
    fn test_not_found() {
        assert_eq!(
            Solution::str_str("china".to_owned(), "taiwan".to_owned()),
            -1
        );
    }

    #[test]
    fn test_empty_needle() {
        assert_eq!(Solution::str_str("hello".to_owned(), "".to_owned()), 0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::str_str("abcdefghijklmnop".to_owned(), "hsnu".to_owned()))
    }
}
