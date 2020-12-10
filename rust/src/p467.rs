/*
Problem 467. Unique Substrings in Wraparound String
===================================================

https://leetcode.com/problems/unique-substrings-in-wraparound-string/

Consider the string s to be the infinite wraparound string of "abcdefghijklmnopqrstuvwxyz", so s
will look like this: "...zabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcd....".

Now we have another string p. Your job is to find out how many unique non-empty substrings of p are
present in s. In particular, your input is the string p and you need to output the number of
different non-empty substrings of p in the string s.

Note: p consists of only lowercase English letters and the size of p might be over 10000.

Example 1:
    Input: "a"
    Output: 1
    Explanation: Only the substring "a" of string "a" is in the string s.

Example 2:
    Input: "cac"
    Output: 2
    Explanation: There are two substrings "a", "c" of string "cac" in the string s.

Example 3:
    Input: "zab"
    Output: 6
    Explanation: There are six substrings "z", "a", "b", "za", "ab", "zab" of string "zab" in the string s.
*/

impl Solution {
    pub fn find_substring_in_wrapround_string(p: String) -> i32 {
        const BASE: u8 = b'a';
        const CHAR_NUM: usize = 26;
        let mut cnt = vec![0; CHAR_NUM];
        let mut exp_chr = b'z' + 1;
        let mut sub_len = 1;
        for cur in p.bytes() {
            if cur == exp_chr {
                sub_len += 1;
            } else {
                sub_len = 1;
            }
            if let Some(c) = cnt.get_mut((cur - BASE) as usize) {
                *c = (*c).max(sub_len);
            }
            exp_chr = (cur - BASE + 1) % CHAR_NUM as u8 + BASE;
        }
        cnt.into_iter().sum()
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
            Solution::find_substring_in_wrapround_string("a".to_owned()),
            1
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::find_substring_in_wrapround_string("cac".to_owned()),
            2
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::find_substring_in_wrapround_string("zab".to_owned()),
            6
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::find_substring_in_wrapround_string(
                "sdoiuewnbadsflkwrnoihhsjdjnlkjwerasdabcdfkeer".to_owned(),
            )
        });
    }
}
