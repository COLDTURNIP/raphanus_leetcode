/*
Problem 52. N-Queens II
=======================

https://leetcode.com/problems/n-queens-ii/

The n-queens puzzle is the problem of placing n queens on an n×n chessboard such that no two queens
attack each other.

    https://assets.leetcode.com/uploads/2018/10/12/8-queens.png

Given an integer n, return the number of distinct solutions to the n-queens puzzle.

Example:

    Input: 4
    Output: 2
    Explanation: There are two distinct solutions to the 4-queens puzzle as shown below.
    [
     [".Q..",  // Solution 1
      "...Q",
      "Q...",
      "..Q."],

     ["..Q.",  // Solution 2
      "Q...",
      "...Q",
      ".Q.."]
    ]
*/

use std::iter::repeat;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        if n < 1 {
            return 0;
        }
        let n = n as usize;
        Self::place_queesn(
            n,
            &mut Vec::new(),
            &mut repeat(false).take(n * 2 - 1).collect::<Vec<bool>>(),
            &mut repeat(false).take(n * 2 - 1).collect::<Vec<bool>>(),
            &mut repeat(false).take(n * 2 - 1).collect::<Vec<bool>>(),
        )
    }

    fn place_queesn(
        n: usize,
        lines: &mut Vec<usize>, // queen position in each line
        xset: &mut Vec<bool>,   // occupied vertical lines
        dset: &mut Vec<bool>,   // occupied diagonals
        aset: &mut Vec<bool>,   // occupied antidiagonals
    ) -> i32 {
        let y = lines.len();
        if y >= n {
            return 1;
        }
        let mut ans = 0;
        for x in 0..n {
            let d_idx = (x + y) % (n * 2 - 1);
            let a_idx = (n + x - y) % (n * 2 - 1);
            if !xset[x] && !dset[d_idx] && !aset[a_idx] {
                lines.push(x);
                xset[x] = true;
                dset[d_idx] = true;
                aset[a_idx] = true;
                ans += Self::place_queesn(n, lines, xset, dset, aset);
                lines.pop();
                xset[x] = false;
                dset[d_idx] = false;
                aset[a_idx] = false;
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

    #[test]
    fn test_4() {
        assert_eq!(Solution::total_n_queens(4), 2);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::total_n_queens(0), 0);
        assert_eq!(Solution::total_n_queens(-1), 0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::total_n_queens(8));
    }
}
