/*
Problem 931. Minimum Falling Path Sum
=====================================

https://leetcode.com/problems/minimum-falling-path-sum/

Given a square array of integers A, we want the minimum sum of a falling path through A.

A falling path starts at any element in the first row, and chooses one element from each row.  The
next row's choice must be in a column that is different from the previous row's column by at most
one.

Example 1:

    Input: [[1,2,3],[4,5,6],[7,8,9]]
    Output: 12
    Explanation:
    The possible falling paths are:
    [1,4,7], [1,4,8], [1,5,7], [1,5,8], [1,5,9]
    [2,4,7], [2,4,8], [2,5,7], [2,5,8], [2,5,9], [2,6,8], [2,6,9]
    [3,5,7], [3,5,8], [3,5,9], [3,6,8], [3,6,9]
    The falling path with the smallest sum is [1,4,7], so the answer is 12.

Constraints:

    - 1 <= A.length == A[0].length <= 100
    - -100 <= A[i][j] <= 100
*/

use std::convert::TryFrom;

impl Solution {
    pub fn min_falling_path_sum(a: Vec<Vec<i32>>) -> i32 {
        let mut last = Vec::new();
        for mut cur in a.into_iter() {
            for (i, entry) in cur.iter_mut().enumerate() {
                let prev_min = [-1, 0, 1]
                    .iter()
                    .flat_map(|d| usize::try_from(i as i32 + d))
                    .flat_map(|i| last.get(i))
                    .min();
                *entry += prev_min.unwrap_or(&0);
            }
            last = cur;
        }
        last.into_iter().min().unwrap_or(0)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_min_path() {
        assert_eq!(
            Solution::min_falling_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            12
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::min_falling_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        });
    }
}
