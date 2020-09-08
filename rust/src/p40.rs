/*
Problem 40. Combination Sum II
==============================

https://leetcode.com/problems/combination-sum-ii/

Given a collection of candidate numbers (candidates) and a target number (target), find all unique
combinations in candidates where the candidate numbers sums to target.

Each number in candidates may only be used once in the combination.

Note:

    All numbers (including target) will be positive integers.
    The solution set must not contain duplicate combinations.

Example 1:

    Input: candidates = [10,1,2,7,6,1,5], target = 8,
    A solution set is:
    [
      [1, 7],
      [1, 2, 5],
      [2, 6],
      [1, 1, 6]
    ]

Example 2:

    Input: candidates = [2,5,2,1,2], target = 5,
    A solution set is:
    [
      [1,2,2],
      [5]
    ]
*/

use std::collections::{BTreeMap, VecDeque};

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let nums = {
            let mut cnt = BTreeMap::new();
            candidates
                .into_iter()
                .for_each(|n| *cnt.entry(n).or_insert(0) += 1);
            cnt
        };
        if target <= 0 || nums.is_empty() {
            return Vec::new();
        }

        #[derive(Debug)]
        struct State {
            rest: i32,
            last: i32,
            comb: Vec<i32>,
        }

        let mut buf = VecDeque::<State>::new();
        buf.push_back(State {
            rest: target,
            last: 0,
            comb: Vec::new(),
        });
        for (n, max) in nums.into_iter() {
            while !buf.is_empty() && buf.front().unwrap().last != n {
                let (mut rest, mut comb) = {
                    let prev_st = buf.pop_front().unwrap();
                    (prev_st.rest, prev_st.comb)
                };
                if rest > 0 && rest < n {
                    continue;
                }
                buf.push_back(State {
                    rest,
                    last: n,
                    comb: comb.clone(),
                });
                for _ in 1..=max {
                    if rest < n {
                        break;
                    }
                    rest -= n;
                    comb.push(n);
                    buf.push_back(State {
                        rest,
                        last: n,
                        comb: comb.clone(),
                    });
                }
            }
        }
        buf.into_iter()
            .filter(|st| st.rest == 0)
            .map(|st| st.comb)
            .collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;
    use std::collections::BTreeSet;

    type Output = Vec<i32>;

    fn check(candidates: Vec<i32>, target: i32, expect: Vec<Output>) {
        let exact = Solution::combination_sum2(candidates, target);
        assert_eq!(
            exact.into_iter().collect::<BTreeSet<Output>>(),
            expect.into_iter().collect::<BTreeSet<Output>>()
        );
    }

    #[test]
    fn test_possible1() {
        check(
            vec![10, 1, 2, 7, 6, 1, 5],
            8,
            vec![vec![1, 7], vec![1, 2, 5], vec![2, 6], vec![1, 1, 6]],
        );
    }

    #[test]
    fn test_possible2() {
        check(vec![2, 5, 2, 1, 2], 5, vec![vec![1, 2, 2], vec![5]]);
    }

    #[test]
    fn test_impossible() {
        check(vec![2, 3, 6, 7], 155, Vec::<Output>::new());
    }

    #[test]
    fn test_no_candidate() {
        check(Vec::new(), 1, Vec::<Output>::new());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::combination_sum2(vec![2, 4, 7, 8, 9, 10], 17));
    }
}
