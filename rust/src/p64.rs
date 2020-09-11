/*
Problem 64. Minimum Path Sum
============================

https://leetcode.com/problems/minimum-path-sum/

Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right
which minimizes the sum of all numbers along its path.

Note: You can only move either down or right at any point in time.

Example:

    Input:
    [
      [1,3,1],
      [1,5,1],
      [4,2,1]
    ]
    Output: 7
    Explanation: Because the path 1→3→1→1→1 minimizes the sum.
*/

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }
        let max_row = grid.len();
        let max_col = grid[0].len();
        let max_move = max_row - 1 + max_col - 1;
        let mut map = grid;
        // loop to tranfer grid into map of minimal path sum
        for mv in 1..=max_move {
            for x in mv.saturating_sub(max_row - 1)..=mv.min(max_col - 1) {
                let y = mv - x;
                map[y][x] += match (x, y) {
                    (0, y) => map[y - 1][0],
                    (x, 0) => map[0][x - 1],
                    (x, y) => map[y - 1][x].min(map[y][x - 1]),
                };
            }
        }
        map[max_row - 1][max_col - 1]
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    #[test]
    fn test_3x3() {
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
    }

    #[test]
    fn test_one_line() {
        assert_eq!(Solution::min_path_sum(vec![vec![1, 0, 0]]), 1);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]));
    }
}
