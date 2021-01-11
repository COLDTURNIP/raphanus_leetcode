/*
Problem 402. Remove K Digits
============================

https://leetcode.com/problems/remove-k-digits/

Given a non-negative integer num represented as a string, remove k digits from the number so that
the new number is the smallest possible.

Note:
    - The length of num is less than 10002 and will be ≥ k.
    - The given num does not contain any leading zero.

Example 1:

    Input: num = "1432219", k = 3
    Output: "1219"
    Explanation: Remove the three digits 4, 3, and 2 to form the new number 1219 which is the
                 smallest.

Example 2:

    Input: num = "10200", k = 1
    Output: "200"
    Explanation: Remove the leading 1 and the number is 200. Note that the output must not contain
                 leading zeroes.

Example 3:

    Input: num = "10", k = 2
    Output: "0"
    Explanation: Remove all the digits from the number and it is left with nothing which is 0.
*/

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let target_len = num.len().saturating_sub(k as usize);
        if num.is_empty() || target_len == 0 {
            return "0".into();
        }
        let k = k as usize;
        let mut ans = num;
        unsafe {
            // Enter the unsafe mode so that we can reuse the existed number string. This problem
            // guarantees the input string as valid ascii number string, so that it is safe to
            // perform our modification under UTF-8 safety.
            let nvec = ans.as_mut_vec();
            let mut shift = 0;
            for ri in 1..nvec.len() {
                let cur = nvec[ri];
                // if current digit is less than the previous digits, remove them from the original
                // number string.
                while shift < k && shift < ri && cur < nvec[ri - shift - 1] {
                    shift += 1;
                }
                nvec[ri - shift] = cur;
            }
            nvec.truncate(target_len);
            if let Some(i) = (0..nvec.len()).find(|&i| nvec[i] != b'0') {
                nvec.drain(0..i);
            } else {
                nvec.truncate(1);
            }
        }
        ans
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
            Solution::remove_kdigits(String::from("1432219"), 3),
            String::from("1219")
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::remove_kdigits(String::from("10200"), 1),
            String::from("200")
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::remove_kdigits(String::from("10"), 2),
            String::from("0")
        );
    }

    #[test]
    fn test_case9() {
        assert_eq!(
            Solution::remove_kdigits(String::from("1234567890"), 9),
            String::from("0")
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            Solution::remove_kdigits(String::from("0"), 3),
            String::from("0")
        );
        assert_eq!(
            Solution::remove_kdigits(String::from("993"), 0),
            String::from("993")
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::remove_kdigits(String::from("1432219"), 3));
    }
}
