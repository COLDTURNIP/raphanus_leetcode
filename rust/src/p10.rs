/*
Problem 10. Regular Expression Matching
=======================================

https://leetcode.com/problems/regular-expression-matching/

Given an input string (s) and a pattern (p), implement regular expression matching with support for
'.' and '*'.

    - '.' Matches any single character.
    - '*' Matches zero or more of the preceding element.
    - The matching should cover the entire input string (not partial).

Note:

    - s could be empty and contains only lowercase letters a-z.
    - p could be empty and contains only lowercase letters a-z, and characters like . or *.

Example 1:

    Input:
        s = "aa"
        p = "a"
    Output: false
    Explanation: "a" does not match the entire string "aa".

Example 2:

    Input:
        s = "aa"
        p = "a*"
    Output: true
    Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".

Example 3:

    Input:
        s = "ab"
        p = ".*"
    Output: true
    Explanation: ".*" means "zero or more (*) of any character (.)".

Example 4:

    Input:
        s = "aab"
        p = "c*a*b"
    Output: true
    Explanation: c can be repeated 0 times, a can be repeated 1 time. Therefore, it matches "aab".

Example 5:

    Input:
        s = "mississippi"
        p = "mis*is*p*."
    Output: false
*/

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if p.is_empty() {
            return s.is_empty();
        }
        // TODO: this solution is DP. Change to 2-cursor scheme to imporove performance.
        enum Pattern {
            Single(char),
            Multi(char),
        }

        let s = s.chars().collect::<Vec<char>>();
        let p = {
            let mut ptn = Vec::new();
            let mut idx = 0;
            let ptn_c = p.chars().collect::<Vec<char>>();
            while let Some(&cur) = ptn_c.get(idx) {
                let next = ptn_c.get(idx + 1);
                if next == Some(&'*') {
                    ptn.push(Pattern::Multi(cur));
                    idx += 2;
                } else {
                    ptn.push(Pattern::Single(cur));
                    idx += 1;
                }
            }
            ptn
        };
        let mut buf_old = vec![false; p.len() + 1];
        let mut buf_new = vec![false; p.len() + 1];
        buf_old[0] = true;
        for (i, pitem) in p.iter().enumerate() {
            buf_old[i + 1] = match pitem {
                Pattern::Multi(_) => buf_old[i],
                _ => false,
            };
        }
        for c in s.into_iter() {
            buf_new[0] = false;
            for (pi, pitem) in p.iter().enumerate() {
                match pitem {
                    Pattern::Multi('.') => buf_new[pi + 1] = buf_old[pi + 1] || buf_new[pi],
                    Pattern::Multi(pc) => {
                        buf_new[pi + 1] = (*pc == c && buf_old[pi + 1]) || buf_new[pi]
                    }
                    Pattern::Single('.') => buf_new[pi + 1] = buf_old[pi],
                    Pattern::Single(pc) => buf_new[pi + 1] = *pc == c && buf_old[pi],
                }
            }
            std::mem::swap(&mut buf_old, &mut buf_new);
        }
        buf_old.last() == Some(&true)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    #[test]
    fn test_always_match() {
        assert!(Solution::is_match("ab".to_owned(), ".*".to_owned()));
    }

    #[test]
    fn test_match_wildcard() {
        assert!(Solution::is_match("aa".to_owned(), "a*".to_owned()));
    }

    #[test]
    fn test_match_wildcard_multiple() {
        assert!(Solution::is_match("aab".to_owned(), "c*a*b".to_owned()));
    }

    #[test]
    fn test_not_match_exact() {
        assert!(!Solution::is_match("aa".to_owned(), "a".to_owned()));
    }

    #[test]
    fn test_not_match_exceed() {
        assert!(!Solution::is_match("a".to_owned(), "aa".to_owned()));
    }

    #[test]
    fn test_not_match_wildcard_multiple() {
        assert!(!Solution::is_match(
            "mississippi".to_owned(),
            "mis*is*p*.".to_owned()
        ));
    }

    #[test]
    fn test_empty() {
        assert!(!Solution::is_match("".to_owned(), "a".to_owned()));
        assert!(!Solution::is_match("aa".to_owned(), "".to_owned()));
        assert!(Solution::is_match("".to_owned(), "".to_owned()));
        assert!(Solution::is_match("".to_owned(), ".*".to_owned()));
        assert!(Solution::is_match("".to_owned(), "a*".to_owned()));
    }

    #[test]
    fn test_wildcard_interleave1() {
        assert!(Solution::is_match("adceb".to_owned(), ".*a.*b".to_owned()));
    }

    #[test]
    fn test_wildcard_interleave2() {
        assert!(Solution::is_match("adceb".to_owned(), "a.*.*b".to_owned()));
    }

    #[test]
    fn test_wildcard_multiple() {
        assert!(!Solution::is_match(
            "bbbaab".to_string(),
            "a**?***".to_string()
        ));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::is_match("abwroijkljsewrovjln".to_owned(), "j?wr?vj".to_owned()));
    }
}
