/*
Problem 67. Add Binary
=====================

https://leetcode.com/problems/add-binary/

Given two binary strings, return their sum (also a binary string).

The input strings are both non-empty and contains only characters 1 or 0.

Example 1:

    Input: a = "11", b = "1"
    Output: "100"

Example 2:

    Input: a = "1010", b = "1011"
    Output: "10101"

Constraints:

    Each string consists only of '0' or '1' characters.
    1 <= a.length, b.length <= 10^4
    Each string is either "0" or doesn't contain any leading zero.
*/

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let max_len = a.len().max(b.len()) + 1;

        let char_to_bit = |c: char| c == '1';
        let bit_to_char = |b: bool| if b { '1' } else { '0' };

        let mut a = a.chars().rev().map(char_to_bit);
        let mut b = b.chars().rev().map(char_to_bit);
        let mut carry = false;
        let mut ans = Vec::with_capacity(max_len);
        loop {
            match (a.next(), b.next()) {
                (Some(da), Some(db)) => {
                    ans.push(bit_to_char(da ^ db ^ carry));
                    carry = if carry { da | db } else { da & db };
                }
                (Some(d), None) | (None, Some(d)) => {
                    ans.push(bit_to_char(d ^ carry));
                    carry &= d;
                }
                (None, None) => {
                    if carry {
                        ans.push(bit_to_char(carry))
                    }
                    break;
                }
            }
        }
        ans.into_iter().rev().collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_add_11_1() {
        assert_eq!(
            Solution::add_binary("11".to_owned(), "1".to_owned()),
            "100".to_owned()
        );
    }

    #[test]
    fn test_add_1010_1011() {
        assert_eq!(
            Solution::add_binary("1010".to_owned(), "1011".to_owned()),
            "10101".to_owned()
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            Solution::add_binary("0".to_owned(), "0".to_owned()),
            "0".to_owned()
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::add_binary("1010".to_owned(), "1011".to_owned()));
    }
}
