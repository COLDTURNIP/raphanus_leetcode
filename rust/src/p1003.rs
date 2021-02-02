/*
Problem 1003. Check If Word Is Valid After Substitutions
========================================================

Given a string s, determine if it is valid.

A string s is valid if, starting with an empty string t = "", you can transform t into s after
performing the following operation any number of times:

    - Insert string "abc" into any position in t. More formally, t becomes tleft + "abc" + tright,
    where t == tleft + tright. Note that tleft and tright may be empty.

Return true if s is a valid string, otherwise, return false.

Example 1:

    Input: s = "aabcbc"
    Output: true
    Explanation:
    "" -> "abc" -> "aabcbc"
    Thus, "aabcbc" is valid.

Example 2:

    Input: s = "abcabcababcc"
    Output: true
    Explanation:
    "" -> "abc" -> "abcabc" -> "abcabcabc" -> "abcabcababcc"
    Thus, "abcabcababcc" is valid.

Example 3:

    Input: s = "abccba"
    Output: false
    Explanation: It is impossible to get "abccba" using the operation.

Example 4:

    Input: s = "cababc"
    Output: false
    Explanation: It is impossible to get "cababc" using the operation.

Constraints:

    - 1 <= s.length <= 2 * 10^4
    - s consists of letters 'a', 'b', and 'c'
*/

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut buf = Vec::with_capacity(s.len() / 3 + 1);
        let mut state: u8 = 0;
        for ch in s.bytes() {
            let cur_c_state = match ch {
                b'a' | b'b' | b'c' => ch - b'a',
                _ => return false,
            };
            if cur_c_state != state {
                state = if cur_c_state == 0 {
                    buf.push(state);
                    0
                } else if let Some(old_state) = buf.pop() {
                    if old_state != cur_c_state {
                        return false;
                    }
                    old_state
                } else {
                    return false;
                }
            }
            state = (state + 1) % 3;
        }
        state == 0 && buf.is_empty()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert!(Solution::is_valid(String::from("aabcbc")));
    }

    #[test]
    fn test_case2() {
        assert!(Solution::is_valid(String::from("abcabcababcc")));
    }

    #[test]
    fn test_case3() {
        assert!(!Solution::is_valid(String::from("abccba")));
    }

    #[test]
    fn test_case4() {
        assert!(!Solution::is_valid(String::from("cababc")));
    }

    #[test]
    fn test_empty() {
        assert!(Solution::is_valid(String::new()));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::is_valid(String::from("abcabcababcc")));
    }
}
