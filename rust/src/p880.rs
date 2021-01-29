/*
Problem 880. Decoded String at Index
====================================

https://leetcode.com/problems/decoded-string-at-index/

An encoded string S is given.  To find and write the decoded string to a tape, the encoded string
is read one character at a time and the following steps are taken:

    - If the character read is a letter, that letter is written onto the tape.
    - If the character read is a digit (say d), the entire current tape is repeatedly written d-1
      more times in total.
    - Now for some encoded string S, and an index K, find and return the K-th letter (1 indexed) in
      the decoded string.

Example 1:

    Input: S = "leet2code3", K = 10
    Output: "o"
    Explanation:
    The decoded string is "leetleetcodeleetleetcodeleetleetcode".
    The 10th letter in the string is "o".

Example 2:

    Input: S = "ha22", K = 5
    Output: "h"
    Explanation:
    The decoded string is "hahahaha".  The 5th letter is "h".

Example 3:

    Input: S = "a2345678999999999999999", K = 1
    Output: "a"
    Explanation:
    The decoded string is "a" repeated 8301530446056247680 times.  The 1st letter is "a".

Constraints:

    2 <= S.length <= 100
    S will only contain lowercase letters and digits 2 through 9.
    S starts with a letter.
    1 <= K <= 10^9
    It's guaranteed that K is less than or equal to the length of the decoded string.
    The decoded string is guaranteed to have less than 2^63 letters.
*/

impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let mut parts: Vec<(
            &str,  // suffix
            usize, // part length
        )> = Vec::new();
        let k = k as usize;
        let decoded_len = {
            // parse encoded string
            let mut cur_part_start = 0;
            let mut total_len: usize = 0;
            let mut cb_iter = s.bytes();
            for i in 0.. {
                match cb_iter.next() {
                    None => {
                        if cur_part_start != i {
                            let suffix = &s[cur_part_start..];
                            total_len = total_len.saturating_add(suffix.len());
                            parts.push((suffix, total_len));
                        }
                        break;
                    }
                    Some(cb) => {
                        if cb.is_ascii_digit() {
                            let suffix = &s[cur_part_start..i];
                            let times = cb - b'0';
                            let part_len = total_len.saturating_add(suffix.len());
                            total_len = part_len.saturating_mul(times as usize);
                            parts.push((suffix, part_len));
                            cur_part_start = i + 1;
                        }
                    }
                }
                if total_len >= k {
                    // no need for further decoding
                    break;
                }
            }
            total_len
        };
        if decoded_len < k || parts.is_empty() {
            return String::new();
        }
        let mut offset = k - 1;
        while let Some((suffix, part_len)) = parts.pop() {
            offset %= part_len;
            if let Some(suffix_offset) = offset.checked_sub(part_len - suffix.len()) {
                return suffix[suffix_offset..suffix_offset + 1].to_string();
            }
        }
        String::new()
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
            Solution::decode_at_index(String::from("leet2code3"), 10),
            String::from("o")
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::decode_at_index(String::from("ha22"), 5),
            String::from("h")
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::decode_at_index(String::from("a2345678999999999999999"), 1),
            String::from("a")
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::decode_at_index(String::new(), 0), String::new());
    }

    #[test]
    fn test_exceed() {
        assert_eq!(
            Solution::decode_at_index(String::from("abcdef"), 999),
            String::new()
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::decode_at_index(String::from("a2345678999999999999999"), 1));
    }
}
