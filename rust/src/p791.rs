/*
Problem 791. Custom Sort String
===============================

S and T are strings composed of lowercase letters. In S, no letter occurs more than once.

S was sorted in some custom order previously. We want to permute the characters of T so that they
match the order that S was sorted. More specifically, if x occurs before y in S, then x should
occur before y in the returned string.

Return any permutation of T (as a string) that satisfies this property.

Example :

    Input:
    S = "cba"
    T = "abcd"
    Output: "cbad"
    Explanation:
    "a", "b", "c" appear in S, so the order of "a", "b", "c" should be "c", "b", and "a".
    Since "d" does not appear in S, it can be at any position in T. "dcba", "cdba", "cbda" are also valid outputs.

Note:

    - S has length at most 26, and no character is repeated in S.
    - T has length at most 200.
    - S and T consist of lowercase letters only.
*/

impl Solution {
    pub fn custom_sort_string(s: String, t: String) -> String {
        let order = {
            let mut order = vec![std::i8::MAX; 26];
            for (i, b) in s.bytes().enumerate() {
                order[(b - b'a') as usize] = i as i8;
            }
            order
        };
        let mut ans = t;
        unsafe {
            ans.as_mut_vec()
                .sort_unstable_by_key(|b| order[(b - b'a') as usize]);
        }
        ans
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;
    use std::collections::HashMap;

    fn check(s: &str, t: &str, expect: &str) {
        let t_cnt = {
            let mut cnt = HashMap::new();
            for c in t.chars() {
                *cnt.entry(c).or_insert(0) += 1;
            }
            cnt
        };
        let exact = Solution::custom_sort_string(s.to_string(), t.to_string());
        let exact_cnt = {
            let mut cnt = HashMap::new();
            for c in exact.chars() {
                *cnt.entry(c).or_insert(0) += 1;
            }
            cnt
        };
        assert_eq!(t_cnt, exact_cnt);
        let mut expect_iter = expect.chars().peekable();
        for exact_c in exact.chars() {
            if Some(&exact_c) == expect_iter.peek() {
                expect_iter.next();
            }
        }
        assert!(expect_iter.peek().is_none());
    }

    #[test]
    fn test_case1() {
        check("cba", "abcd", "cba");
    }

    #[test]
    fn test_case2() {
        check("exv", "xwvee", "eexv");
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::custom_sort_string(String::from("cba"), String::from("abcd")));
    }
}
