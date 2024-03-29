/*
Problem 63. Unique Paths II
===========================

https://leetcode.com/problems/unique-paths-ii/

A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).

The robot can only move either down or right at any point in time. The robot is trying to reach the
bottom-right corner of the grid (marked 'Finish' in the diagram below).

Now consider if some obstacles are added to the grids. How many unique paths would there be?

https://assets.leetcode.com/uploads/2018/10/22/robot_maze.png
An obstacle and empty space is marked as 1 and 0 respectively in the grid.

Note: m and n will be at most 100.

Example 1:

    Input:
    [
      [0,0,0],
      [0,1,0],
      [0,0,0]
    ]
    Output: 2
    Explanation:
    There is one obstacle in the middle of the 3x3 grid above.
    There are two ways to reach the bottom-right corner:
    1. Right -> Right -> Down -> Down
    2. Down -> Down -> Right -> Right
*/

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid.is_empty() || obstacle_grid[0].is_empty() {
            return 0;
        }
        let max_row = obstacle_grid.len();
        let max_col = obstacle_grid[0].len();
        let max_move = max_row - 1 + max_col - 1;
        let mut map = obstacle_grid;
        // loop to tranfer obstacle_grid into map of reachable path counters
        for mv in 0..=max_move {
            for x in mv.saturating_sub(max_row - 1)..=mv.min(max_col - 1) {
                let y = mv - x;
                map[y][x] = match (x, y) {
                    (x, y) if map[y][x] == 1 => 0,
                    (0, 0) => 1,
                    (0, y) => map[y - 1][0],
                    (x, 0) => map[0][x - 1],
                    (x, y) => map[y - 1][x] + map[y][x - 1],
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
    fn test_3x3_possible() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]),
            2
        );
    }

    #[test]
    fn test_3x3_impossible() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![1, 0, 1]
            ]),
            0
        );
    }

    #[test]
    fn test_impossible() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![1, 0, 0]]),
            0
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]])
        });
    }
}
