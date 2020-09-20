/*
Problem 96. Unique Binary Search Trees
======================================

https://leetcode.com/problems/unique-binary-search-trees/

Given n, how many structurally unique BST's (binary search trees) that store values 1 ... n?

Example:

    Input: 3
    Output: 5
    Explanation:
    Given n = 3, there are a total of 5 unique BST's:

       1         3     3      2      1
        \       /     /      / \      \
         3     2     1      1   3      2
        /     /       \                 \
       2     1         2                 3

Constraints:

    1 <= n <= 19
*/

use std::collections::HashMap;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut cache = HashMap::new();
        Self::calc_num_trees(n, &mut cache)
    }

    fn calc_num_trees(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
        if n <= 2 {
            n.max(1)
        } else if let Some(ans) = cache.get(&n) {
            *ans
        } else {
            let cnt = (0..=n - 1)
                .map(|left| {
                    Self::calc_num_trees(left, cache) * Self::calc_num_trees(n - 1 - left, cache)
                })
                .sum();
            cache.insert(n, cnt);
            cnt
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    #[test]
    fn test_3() {
        assert_eq!(Solution::num_trees(3), 5);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::num_trees(5), 42);
    }

    #[test]
    fn test_single_comb() {
        assert_eq!(Solution::num_trees(0), 1);
        assert_eq!(Solution::num_trees(1), 1);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::num_trees(6));
    }
}
