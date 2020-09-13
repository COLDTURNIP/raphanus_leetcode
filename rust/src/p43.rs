/*
Problem 43. Multiply Strings
============================

https://leetcode.com/problems/multiply-strings/

Given two non-negative integers num1 and num2 represented as strings, return the product of num1
and num2, also represented as a string.

Example 1:

    Input: num1 = "2", num2 = "3"
    Output: "6"

Example 2:

    Input: num1 = "123", num2 = "456"
    Output: "56088"

Note:

    The length of both num1 and num2 is < 110.
    Both num1 and num2 contain only digits 0-9.
    Both num1 and num2 do not contain any leading zero, except the number 0 itself.
    You must not use any built-in BigInteger library or convert the inputs to integer directly.
*/

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1.is_empty() || num2.is_empty() || num1 == "0" || num2 == "0" {
            return "0".to_owned();
        }
        let n1 = num1
            .chars()
            .rev()
            .map(Self::char_to_int)
            .collect::<Vec<u32>>();
        let n2 = num2
            .chars()
            .rev()
            .map(Self::char_to_int)
            .collect::<Vec<u32>>();
        let (l1, l2) = (n1.len(), n2.len());
        let mut mul = Vec::<Vec<u32>>::with_capacity(n1.len());
        for &d1 in n1.iter() {
            let row = n2.iter().map(|&d2| d1 * d2).collect();
            mul.push(row);
        }
        let mut ans = Vec::new();
        let mut carry = 0;
        for pos in 0..=l1 - 1 + l2 - 1 {
            for (ri, row) in mul
                .iter()
                .enumerate()
                .take(pos.min(l1 - 1) + 1)
                .skip(pos.saturating_sub(l2 - 1))
            {
                let ci = pos - ri;
                carry += row[ci];
            }
            ans.push(Self::int_to_string(carry % 10));
            carry /= 10;
        }
        while carry > 0 {
            ans.push(Self::int_to_string(carry % 10));
            carry /= 10;
        }
        ans.into_iter().rev().collect()
    }

    fn char_to_int(c: char) -> u32 {
        c.to_digit(10).unwrap()
    }

    fn int_to_string(r: u32) -> String {
        r.to_string()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_2x3() {
        assert_eq!(
            Solution::multiply("2".to_owned(), "3".to_owned()),
            "6".to_owned()
        );
    }

    #[test]
    fn test_123x4567() {
        assert_eq!(
            Solution::multiply("123".to_owned(), "4567".to_owned()),
            "561741".to_owned()
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(
            Solution::multiply(String::new(), "3".to_owned()),
            "0".to_owned()
        );
        assert_eq!(
            Solution::multiply("2".to_owned(), String::new()),
            "0".to_owned()
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            Solution::multiply("0".to_owned(), "3".to_owned()),
            "0".to_owned()
        );
        assert_eq!(
            Solution::multiply("2".to_owned(), "0".to_owned()),
            "0".to_owned()
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::multiply("1234".to_owned(), "456".to_owned()));
    }
}
