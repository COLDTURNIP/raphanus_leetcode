/*
Problem 443. String Compression
===============================

Given an array of characters chars, compress it using the following algorithm:

Begin with an empty string s. For each group of consecutive repeating characters in chars:

    - If the group's length is 1, append the character to s.
    - Otherwise, append the character followed by the group's length.

The compressed string s should not be returned separately, but instead be stored in the input
character array chars. Note that group lengths that are 10 or longer will be split into multiple
characters in chars.

After you are done modifying the input array, return the new length of the array.

Follow up:
    Could you solve it using only O(1) extra space?

Example 1:

    Input: chars = ["a","a","b","b","c","c","c"]
    Output: Return 6, and the first 6 characters of the input array should be: ["a","2","b","2","c","3"]
    Explanation: The groups are "aa", "bb", and "ccc". This compresses to "a2b2c3".

Example 2:

    Input: chars = ["a"]
    Output: Return 1, and the first character of the input array should be: ["a"]
    Explanation: The only group is "a", which remains uncompressed since it's a single character.

Example 3:

    Input: chars = ["a","b","b","b","b","b","b","b","b","b","b","b","b"]
    Output: Return 4, and the first 4 characters of the input array should be: ["a","b","1","2"].
    Explanation: The groups are "a" and "bbbbbbbbbbbb". This compresses to "ab12".

Example 4:

    Input: chars = ["a","a","a","b","b","a","a"]
    Output: Return 6, and the first 6 characters of the input array should be: ["a","3","b","2","a","2"].
    Explanation: The groups are "aaa", "bb", and "aa". This compresses to "a3b2a2". Note that each
    group is independent even if two groups have the same character.

Constraints:

    - 1 <= chars.length <= 2000
    - chars[i] is a lower-case English letter, upper-case English letter, digit, or symbol.
*/

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let (mut wi, mut ri) = (0, 0);
        let mut last_chr = None;
        let mut last_cnt = 0;

        fn write_cnt(target: &mut Vec<char>, start: usize, mut cnt: i32) -> usize {
            if cnt <= 1 {
                0
            } else {
                let mut cnt_len = 0;
                while cnt > 0 {
                    target[start + cnt_len] = ((cnt % 10) as u8 + b'0').into();
                    cnt /= 10;
                    cnt_len += 1;
                }
                target[start..start + cnt_len].reverse();
                cnt_len
            }
        }

        loop {
            let cur_chr = chars.get(ri).cloned();
            match cur_chr {
                None => break (wi + write_cnt(chars, wi, last_cnt)) as i32,
                Some(c) if cur_chr != last_chr => {
                    wi += write_cnt(chars, wi, last_cnt);
                    chars[wi] = c;
                    wi += 1;
                    last_chr = cur_chr;
                    last_cnt = 1;
                }
                _ => last_cnt += 1,
            }
            ri += 1;
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    fn check(input: &str, expect: &str) {
        let mut exact: Vec<char> = input.chars().collect();
        let expect_vec: Vec<char> = expect.chars().collect();
        assert_eq!(Solution::compress(&mut exact), expect_vec.len() as i32);
        assert_eq!(exact[0..expect_vec.len()].to_vec(), expect_vec);
    }

    #[test]
    fn test_case1() {
        check("aabbccc", "a2b2c3");
    }

    #[test]
    fn test_case2() {
        check("a", "a");
    }

    #[test]
    fn test_case3() {
        check("abbbbbbbbbbbb", "ab12");
    }

    #[test]
    fn test_case4() {
        check("aaabbaa", "a3b2a2");
    }

    #[test]
    fn test_empty() {
        check("", "");
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::compress(&mut "abbbbbbbbbbbb".chars().collect()));
    }
}
