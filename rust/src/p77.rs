/*
Problem 77. Combinations
========================

https://leetcode.com/problems/combinations/

Given two integers n and k, return all possible combinations of k numbers out of 1 ... n.

You may return the answer in any order.

Example 1:

    Input: n = 4, k = 2
    Output:
    [
      [2,4],
      [3,4],
      [2,3],
      [1,2],
      [1,3],
      [1,4],
    ]

Example 2:

    Input: n = 1, k = 1
    Output: [[1]]

Constraints:

    1 <= n <= 20
    1 <= k <= n
*/

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        if k < 1 {
            return Vec::new();
        }
        let k = k as usize;
        // fact 1: If k-1th digit equals to n, then the k-2th digit must be increased, and the k-1th
        //         digit should be reset to k-2th digit plus 1. For k-2th digit, the maximum
        //         possible number is n-1... For digit ith, the maximum possible number is n-k+1+i
        // fact 2: If the 1st digit hits n-1, then we've iterated all of the combinations. In other
        //         word, the 1st digit must always less than n.
        let mut ans = Vec::new();
        let mut state = (0..k).map(|n| n as i32 + 1).collect::<Vec<i32>>();
        let mut cur = k - 1;
        'iter_comb: loop {
            ans.push(state.clone());
            while state[cur] >= n - k as i32 + 1 + cur as i32 {
                if let Some(next_cur) = cur.checked_sub(1) {
                    cur = next_cur
                } else {
                    break 'iter_comb;
                }
            }
            state[cur] += 1;
            while cur < k - 1 {
                state[cur + 1] = state[cur] + 1;
                cur += 1;
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
    use std::collections::HashSet;
    use std::iter::FromIterator;

    fn check(n: i32, k: i32, expect_set_size: usize) {
        let mut result_set = HashSet::new();
        for mut comb in Solution::combine(n, k).into_iter() {
            comb.sort_unstable();
            assert!(comb.iter().min().cloned().unwrap() >= 0);
            assert!(comb.iter().max().cloned().unwrap() <= n);
            assert!(HashSet::<i32>::from_iter(comb.clone()).len() == k as usize);
            result_set.insert(comb);
        }
        assert_eq!(result_set.len(), expect_set_size);
    }

    #[test]
    fn test_4_2() {
        check(4, 2, 6);
    }

    #[test]
    fn test_1_1() {
        check(1, 1, 1)
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::combine(13, 4));
    }
}
