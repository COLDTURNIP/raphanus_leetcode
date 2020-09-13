/*
Problem 74. Search a 2D Matrix
==============================

Write an efficient algorithm that searches for a value in an m x n matrix. This matrix has the
following properties:

    - Integers in each row are sorted from left to right.
    - The first integer of each row is greater than the last integer of the previous row.

Example 1:

    Input:
    matrix = [
      [1,   3,  5,  7],
      [10, 11, 16, 20],
      [23, 30, 34, 50]
    ]
    target = 3
    Output: true

Example 2:

    Input:
    matrix = [
      [1,   3,  5,  7],
      [10, 11, 16, 20],
      [23, 30, 34, 50]
    ]
    target = 13
    Output: false
*/

use std::cmp::Ordering::{Equal, Greater, Less};

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let rows = matrix
            .iter()
            .filter(|v| !v.is_empty())
            .collect::<Vec<&Vec<i32>>>();

        // boundary check, ensure target falls in our ranges
        if rows.is_empty()
            || rows
                .first()
                .and_then(|v| v.first().filter(|&&n| n <= target))
                .is_none()
            || rows
                .last()
                .and_then(|v| v.last().filter(|&&n| n >= target))
                .is_none()
        {
            return false;
        }

        // binray search
        rows.binary_search_by(|v| {
            let lcmp = v.first().unwrap().cmp(&target);
            let rcmp = v.last().unwrap().cmp(&target);
            match (lcmp, rcmp) {
                (Less, Less) => Less,
                (Greater, Greater) => Greater,
                (Greater, Equal) | (Greater, Less) | (Equal, Less) => unreachable!(),
                _ => Equal,
            }
        })
        .and_then(|idx| rows[idx].binary_search(&target))
        .is_ok()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_true() {
        assert!(Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
            3
        ))
    }

    #[test]
    fn test_false() {
        assert!(!Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16], vec![23, 30, 34, 50]],
            13
        ))
    }

    #[test]
    fn test_empty() {
        assert!(!Solution::search_matrix(Vec::new(), 13))
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                3,
            )
        });
    }
}
