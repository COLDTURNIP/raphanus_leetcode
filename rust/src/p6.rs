/*
Problem 6. ZigZag Conversion
============================

https://leetcode.com/problems/zigzag-conversion/

The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this:
(you may want to display this pattern in a fixed font for better legibility)

    P   A   H   N
    A P L S I I G
    Y   I   R

And then read line by line: "PAHNAPLSIIGYIR"

Write the code that will take a string and make this conversion given a number of rows:

string convert(string s, int numRows);

Example 1:

    Input: s = "PAYPALISHIRING", numRows = 3
    Output: "PAHNAPLSIIGYIR"

Example 2:

    Input: s = "PAYPALISHIRING", numRows = 4
    Output: "PINALSIGYAHRPI"

Explanation:

    P     I    N
    A   L S  I G
    Y A   H R
    P     I
*/

use std::iter::{successors, Iterator};

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let s = s.chars().collect::<Vec<char>>();
        let len = s.len();
        let num_rows = num_rows as usize;
        let period = (num_rows - 1) * 2;
        (0..num_rows)
            .flat_map(|line| {
                Box::new(
                    successors(Some(line), move |i| {
                        if line == 0 || line == num_rows - 1 {
                            Some(i + period)
                        } else if i % period == line {
                            Some(i + period - line * 2)
                        } else {
                            Some(i + line * 2)
                        }
                    })
                    .take_while(|&i| i < len),
                ) as Box<dyn Iterator<Item = usize>>
            })
            .map(|idx| s[idx])
            .collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_3() {
        // A   E   I   M
        // B D F H J L N
        // C   G   K
        assert_eq!(
            Solution::convert("ABCDEFGHIJKLMN".to_owned(), 3),
            "AEIMBDFHJLNCGK".to_owned()
        );
    }

    #[test]
    fn test_4() {
        // A     G     M
        // B   F H   L N
        // C E   I K
        // D     J
        assert_eq!(
            Solution::convert("ABCDEFGHIJKLMN".to_owned(), 4),
            "AGMBFHLNCEIKDJ".to_owned()
        );
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::convert("A".to_owned(), 4), "A".to_owned());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::convert("PAYPALISHIRING".to_owned(), 4))
    }
}
