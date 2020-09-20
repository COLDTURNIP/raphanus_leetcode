/*
Problem 22. Generate Parentheses
================================

https://leetcode.com/problems/generate-parentheses/

Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

For example, given n = 3, a solution set is:

[
  "((()))",
  "(()())",
  "(())()",
  "()(())",
  "()()()"
]
*/

use std::collections::VecDeque;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n < 1 {
            return Vec::new();
        }
        let mut ans: VecDeque<(i32, i32, String)> = VecDeque::with_capacity(1 << n as usize);
        ans.push_back((0, 0, String::new()));
        for _ in 1..=n as usize * 2 {
            for _ in 0..ans.len() {
                let (lc, rc, mut comb) = ans.pop_front().unwrap();
                if rc < lc {
                    let mut comb = comb.clone();
                    comb.push(')');
                    ans.push_back((lc, rc + 1, comb));
                }
                if lc < n {
                    comb.push('(');
                    ans.push_back((lc + 1, rc, comb));
                }
            }
        }
        ans.into_iter().map(|t| t.2).collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;
    use std::collections::HashSet;

    fn check(input: i32, expect: Vec<&'static str>) {
        let exact = Solution::generate_parenthesis(input);
        let exact_set = exact.clone().into_iter().collect::<HashSet<String>>();
        assert_eq!(exact.len(), exact_set.len(), "duplicated output");
        println!("n={}, exact={:?}", input, exact);
        let expact = expect
            .into_iter()
            .map(String::from)
            .collect::<HashSet<String>>();
        assert_eq!(exact_set, expact);
    }

    #[test]
    fn test_3() {
        check(3, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]);
    }

    #[test]
    fn test_zero() {
        check(0, Vec::new());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::generate_parenthesis(10));
    }
}
