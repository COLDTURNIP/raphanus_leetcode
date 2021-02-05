/*
Problem 1081. Smallest Subsequence of Distinct Characters
=========================================================

https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/

Return the lexicographically smallest subsequence of s that contains all the distinct characters of
s exactly once.

Note: This question is the same as 316: https://leetcode.com/problems/remove-duplicate-letters/

Example 1:

    Input: s = "bcabc"
    Output: "abc"

Example 2:

    Input: s = "cbacdcbc"
    Output: "acdb"

Constraints:

    - 1 <= s.length <= 1000
    - s consists of lowercase English letters.
*/

impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let mut ch_cnt = {
            let mut cnt = vec![0; 26];
            for bc in s.bytes() {
                cnt[(bc - b'a') as usize] += 1;
            }
            cnt
        };

        let mut inserted = vec![false; 26];
        let mut ans = Vec::with_capacity(ch_cnt.len());
        for bc in s.bytes() {
            let bci = (bc - b'a') as usize;
            ch_cnt[bci] -= 1;
            if inserted[bci] {
                continue;
            }
            let mut ans_len = ans.len();
            for i in (0..ans_len).rev() {
                let tail = ans[i];
                let taili = (tail - b'a') as usize;
                if ch_cnt[taili] > 0 && bc <= tail {
                    inserted[taili] = false;
                    ans_len = i;
                } else {
                    break;
                }
            }
            ans.truncate(ans_len);
            ans.push(bc);
            inserted[bci] = true;
        }
        String::from_utf8(ans).expect("invalid result bytes array")
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
            Solution::smallest_subsequence(String::from("bcabc")),
            String::from("abc")
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::smallest_subsequence(String::from("cbacdcbc")),
            String::from("acdb")
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::smallest_subsequence(String::from("cbaacabcaaccaacababa")),
            String::from("abc")
        );
    }

    #[test]
    fn test_case4() {
        assert_eq!(
            Solution::smallest_subsequence(String::from("bcbcbcababa")),
            String::from("bca")
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::smallest_subsequence(String::new()), String::new());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::smallest_subsequence(String::from("saflfkhwrenlkjaalsdkfj")));
    }
}
