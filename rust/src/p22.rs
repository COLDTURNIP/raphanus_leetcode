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

use std::collections::{HashMap, HashSet};

struct SolutionCache(HashMap<i32, HashSet<String>>);

impl SolutionCache {
    fn new() -> Self {
        let mut cache = HashMap::new();
        cache.insert(0, HashSet::new());
        cache.insert(
            1,
            vec!["()".to_string()]
                .into_iter()
                .collect::<HashSet<String>>(),
        );
        Self(cache)
    }

    fn get_solution(&mut self, n: i32) -> HashSet<String> {
        match self.0.get(&n) {
            Some(comb) => comb.clone(),
            None => {
                let mut comb = HashSet::new();
                for sub in self.get_solution(n - 1) {
                    println!("pushing wrapped: ({})", sub);
                    comb.insert(format!("({})", sub));
                }
                for a in 1..n {
                    let head_comb = self.get_solution(a);
                    let tail_comb = self.get_solution(n - a);
                    for head in head_comb.iter() {
                        for tail in tail_comb.iter() {
                            println!("pushing combined: {}+{}", head, tail);
                            comb.insert(format!("{}{}", head, tail));
                        }
                    }
                }
                self.0.insert(n, comb.clone());
                comb
            }
        }
    }
}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut cache = SolutionCache::new();
        cache.get_solution(n).into_iter().collect()
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
