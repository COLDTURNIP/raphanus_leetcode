/*
Problem 344. Reverse String
===========================

https://leetcode.com/problems/reverse-string/

Write a function that reverses a string. The input string is given as an array of characters
char[].

Do not allocate extra space for another array, you must do this by modifying the input array
in-place with O(1) extra memory.

You may assume all the characters consist of printable ascii characters.

Example 1:

    Input: ["h","e","l","l","o"]
    Output: ["o","l","l","e","h"]

Example 2:

    Input: ["H","a","n","n","a","h"]
    Output: ["h","a","n","n","a","H"]
*/

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        s.reverse();
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    fn check(input: &str, expect: &str) {
        let mut input_vec: Vec<char> = input.chars().collect();
        Solution::reverse_string(&mut input_vec);
        assert_eq!(input_vec, expect.chars().collect::<Vec<char>>());
    }

    #[test]
    fn test_hello() {
        check("hello", "olleh");
    }

    #[test]
    fn test_hannah() {
        check("Hannah", "hannaH");
    }

    #[test]
    fn test_empty() {
        check("", "");
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::reverse_string(&mut "alskfj;lasjdfoeiwrjlk".chars().collect::<Vec<char>>())
        });
    }
}
