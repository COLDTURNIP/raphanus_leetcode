/*
Problem 73. Set Matrix Zeroes
=============================

https://leetcode.com/problems/set-matrix-zeroes/

Given an m x n matrix. If an element is 0, set its entire row and column to 0. Do it in-place.

Follow up:

A straight forward solution using O(mn) space is probably a bad idea.
A simple improvement uses O(m + n) space, but still not the best solution.
Could you devise a constant space solution?


Example 1:

    Input: matrix = [[1,1,1],
                     [1,0,1],
                     [1,1,1]]
    Output: [[1,0,1],
             [0,0,0],
             [1,0,1]]

Example 2:

    Input: matrix = [[0,1,2,0],
                     [3,4,5,2],
                     [1,3,1,5]]
    Output: [[0,0,0,0],
             [0,4,5,0],
             [0,3,1,0]]
*/

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        if matrix.is_empty() || matrix[0].is_empty() {
            return;
        }
        let max_row = matrix.len();
        let max_col = matrix[0].len();
        let clear_head_col = matrix.iter().any(|row| row[0] == 0);
        let clear_head_row = matrix[0].iter().any(|&val| val == 0);
        for y in 1..max_row {
            for x in 1..max_col {
                if matrix[y][x] == 0 {
                    matrix[0][x] = 0;
                    matrix[y][0] = 0;
                }
            }
        }
        for y in 1..max_row {
            for x in 1..max_col {
                if matrix[y][0] == 0 || matrix[0][x] == 0 {
                    matrix[y][x] = 0;
                }
            }
        }
        if clear_head_col {
            matrix.iter_mut().for_each(|row| row[0] = 0);
        }
        if clear_head_row {
            matrix[0].iter_mut().for_each(|val| *val = 0);
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    fn check(mut input: Vec<Vec<i32>>, expect: Vec<Vec<i32>>) {
        Solution::set_zeroes(&mut input);
        assert_eq!(input, expect);
    }

    #[test]
    fn test_3x3() {
        check(
            vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]],
            vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]],
        );
    }

    #[test]
    fn test_3x4() {
        check(
            vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]],
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]],
        );
    }

    #[test]
    fn test_4x4() {
        check(
            vec![
                vec![1, 2, 3, 4],
                vec![5, 0, 7, 8],
                vec![0, 10, 11, 12],
                vec![13, 14, 15, 0],
            ],
            vec![
                vec![0, 0, 3, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ],
        );
    }

    #[test]
    fn test_empty() {
        check(Vec::new(), Vec::new());
        check(vec![Vec::new()], vec![Vec::new()]);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::set_zeroes(&mut vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]));
    }
}
