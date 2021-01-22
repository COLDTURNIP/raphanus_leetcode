/*
Problem 394. Decode String
==========================

https://leetcode.com/problems/decode-string/

Given an encoded string, return its decoded string.

The encoding rule is: k[encoded_string], where the encoded_string inside the square brackets is
being repeated exactly k times. Note that k is guaranteed to be a positive integer.

You may assume that the input string is always valid; No extra white spaces, square brackets are
well-formed, etc.

Furthermore, you may assume that the original data does not contain any digits and that digits are
only for those repeat numbers, k. For example, there won't be input like 3a or 2[4].

Example 1:

    Input: s = "3[a]2[bc]"
    Output: "aaabcbc"

Example 2:

    Input: s = "3[a2[c]]"
    Output: "accaccacc"

Example 3:

    Input: s = "2[abc]3[cd]ef"
    Output: "abcabccdcdcdef"

Example 4:

    Input: s = "abc3[cd]xyz"
    Output: "abccdcdcdxyz"

Constraints:

    - 1 <= s.length <= 30
    - s consists of lowercase English letters, digits, and square brackets '[]'.
    - s is guaranteed to be a valid input.
    - All the integers in s are in the range [1, 300].
*/

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut ptn_buf = vec![String::new()];
        let mut times_buf = Vec::new();
        let mut times = 0;
        for c in s.chars() {
            match (c, c.to_digit(10)) {
                (_, Some(n)) => times = times * 10 + n,
                ('[', None) => {
                    times_buf.push(times);
                    ptn_buf.push(String::new());
                    times = 0;
                }
                (']', None) => {
                    let times = times_buf.pop().unwrap();
                    let ptn = ptn_buf.pop().unwrap();
                    if let Some(last_ptn) = ptn_buf.last_mut() {
                        last_ptn.push_str(&ptn.repeat(times as usize));
                    }
                }
                _ => {
                    if let Some(last_ptn) = ptn_buf.last_mut() {
                        last_ptn.push(c);
                    }
                }
            }
        }
        ptn_buf.pop().unwrap()
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
            Solution::decode_string(String::from("3[a]2[bc]")),
            String::from("aaabcbc")
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::decode_string(String::from("3[a2[c]]")),
            String::from("accaccacc")
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::decode_string(String::from("2[abc]3[cd]ef")),
            String::from("abcabccdcdcdef")
        );
    }

    #[test]
    fn test_case4() {
        assert_eq!(
            Solution::decode_string(String::from("abc3[cd]xyz")),
            String::from("abccdcdcdxyz")
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::decode_string(String::from("3[a2[c]]")));
    }
}
