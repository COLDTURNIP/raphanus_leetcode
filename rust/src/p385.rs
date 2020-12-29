/*
Problem 385. Mini Parser
========================

https://leetcode.com/problems/mini-parser/

Given a nested list of integers represented as a string, implement a parser to deserialize it.

Each element is either an integer, or a list -- whose elements may also be integers or other lists.

Note: You may assume that the string is well-formed:

    - String is non-empty.
    - String does not contain white spaces.
    - String contains only digits 0-9, [, - ,, ].

Example 1:

    Given s = "324",

    You should return a NestedInteger object which contains a single integer 324.

Example 2:

    Given s = "[123,[456,[789]]]",

    Return a NestedInteger object containing a nested list with 2 elements:

    1. An integer containing value 123.
    2. A nested list containing two elements:
        i.  An integer containing value 456.
        ii. A nested list with one element:
             a. An integer containing value 789.
*/

use std::iter::Peekable;

impl Solution {
    fn parse_entry(scanner: &mut Peekable<std::str::Chars>) -> Option<NestedInteger> {
        if let Some(&c) = scanner.peek() {
            match c {
                '[' => Some(NestedInteger::List(Self::parse_list(scanner))),
                '-' => Some(NestedInteger::Int(Self::parse_num(scanner))),
                c if c.is_ascii_digit() => Some(NestedInteger::Int(Self::parse_num(scanner))),
                _ => None,
            }
        } else {
            None
        }
    }

    fn parse_num(scanner: &mut Peekable<std::str::Chars>) -> i32 {
        let sign = if *scanner.peek().unwrap() != '-' {
            1
        } else {
            scanner.next();
            -1
        };
        let mut n = 0;
        while let Some(&c) = scanner.peek() {
            if let Some(d) = c.to_digit(10) {
                n = n * 10 + d as i32;
            } else {
                break;
            }
            scanner.next();
        }
        n * sign
    }

    fn parse_list(scanner: &mut Peekable<std::str::Chars>) -> Vec<NestedInteger> {
        assert_eq!(scanner.next(), Some('['));
        let mut ret = Vec::new();
        loop {
            if let Some(entry) = Self::parse_entry(scanner) {
                ret.push(entry);
            }
            if scanner.next() == Some(']') {
                break ret;
            }
        }
    }

    pub fn deserialize(s: String) -> NestedInteger {
        Self::parse_entry(&mut s.chars().peekable()).unwrap()
    }
}

pub struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::{NestedInteger, Solution};

    #[test]
    fn test_single_number() {
        assert_eq!(
            Solution::deserialize("324".to_owned()),
            NestedInteger::Int(324)
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(Solution::deserialize("0".to_owned()), NestedInteger::Int(0));
        assert_eq!(
            Solution::deserialize("-0".to_owned()),
            NestedInteger::Int(0)
        );
    }

    #[test]
    fn test_empty_list() {
        assert_eq!(
            Solution::deserialize("[]".to_owned()),
            NestedInteger::List(Vec::new())
        );
    }

    #[test]
    fn test_single_negative() {
        assert_eq!(
            Solution::deserialize("-21234".to_owned()),
            NestedInteger::Int(-21234)
        );
    }

    #[test]
    fn test_nested() {
        assert_eq!(
            Solution::deserialize("[123,[456,[789]]]".to_owned()),
            NestedInteger::List(vec![
                NestedInteger::Int(123),
                NestedInteger::List(vec![
                    NestedInteger::Int(456),
                    NestedInteger::List(vec![NestedInteger::Int(789)]),
                ]),
            ])
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::deserialize("[123,[456,[789]]]".to_owned()));
    }
}
