/*
Problem 39. Combination Sum
===========================

https://leetcode.com/problems/combination-sum/

Given a set of candidate numbers (candidates) (without duplicates) and a target number (target),
find all unique combinations in candidates where the candidate numbers sums to target.

The same repeated number may be chosen from candidates unlimited number of times.

Note:

    All numbers (including target) will be positive integers.
    The solution set must not contain duplicate combinations.

Example 1:

    Input: candidates = [2,3,6,7], target = 7,
    A solution set is:
    [
      [7],
      [2,2,3]
    ]

Example 2:

    Input: candidates = [2,3,5], target = 8,
    A solution set is:
    [
      [2,2,2,2],
      [2,3,3],
      [3,5]
    ]

Constraints:

    1 <= candidates.length <= 30
    1 <= candidates[i] <= 200
    Each element of candidate is unique.
    1 <= target <= 500
*/

use std::collections::VecDeque;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let nums = {
            let mut ns = candidates;
            ns.sort_unstable();
            ns
        };
        if target <= 0 || nums.is_empty() || target < nums[0] {
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
            last: nums[0] - 1,
            comb: Vec::new(),
        });
        for n in nums.into_iter() {
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
                while rest >= n {
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
        let exact = Solution::combination_sum(candidates, target);
        assert_eq!(
            exact.into_iter().collect::<BTreeSet<Output>>(),
            expect.into_iter().collect::<BTreeSet<Output>>()
        );
    }

    #[test]
    fn test_2367_to_7() {
        check(vec![2, 3, 6, 7], 7, vec![vec![2, 2, 3], vec![7]]);
    }

    #[test]
    fn test_235_to_8() {
        check(
            vec![2, 3, 5],
            8,
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]],
        );
    }

    #[test]
    fn test_impossible() {
        check(vec![2, 3, 6, 7], 1, Vec::<Output>::new());
    }

    #[test]
    fn test_no_candidate() {
        check(Vec::new(), 1, Vec::<Output>::new());
    }

    //#[bench]
    //fn bench(b: &mut test::Bencher) {
    //    b.iter(|| Solution::combination_sum(vec![2, 4, 7, 8, 9, 10], 234));
    //}
}
