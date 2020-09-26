/*
Problem 51. N-Queens
====================

https://leetcode.com/problems/n-queens/

The n-queens puzzle is the problem of placing n queens on an n×n chessboard such that no two queens
attack each other.

    https://assets.leetcode.com/uploads/2018/10/12/8-queens.png

Given an integer n, return all distinct solutions to the n-queens puzzle.

Each solution contains a distinct board configuration of the n-queens' placement, where 'Q' and '.'
both indicate a queen and an empty space respectively.

Example:

    Input: 4
    Output: [
     [".Q..",  // Solution 1
      "...Q",
      "Q...",
      "..Q."],

     ["..Q.",  // Solution 2
      "Q...",
      "...Q",
      ".Q.."]
    ]
    Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above.
*/

use std::iter::repeat;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        if n < 1 {
            return Vec::new();
        }
        let n = n as usize;
        let mut ans = Vec::new();
        Self::place_queesn(
            n,
            &mut Vec::new(),
            &mut repeat(false).take(n * 2).collect::<Vec<bool>>(),
            &mut repeat(false).take(n * 2).collect::<Vec<bool>>(),
            &mut repeat(false).take(n * 2).collect::<Vec<bool>>(),
            &mut ans,
        );
        ans
    }

    fn place_queesn(
        n: usize,
        lines: &mut Vec<usize>, // queen position in each line
        xset: &mut Vec<bool>,   // occupied vertical lines
        dset: &mut Vec<bool>,   // occupied diagonals
        aset: &mut Vec<bool>,   // occupied antidiagonals
        ans: &mut Vec<Vec<String>>,
    ) {
        let y = lines.len();
        if y >= n {
            ans.push(lines.iter().map(|&pos| Self::build_lint(n, pos)).collect());
            return;
        }
        for x in 0..n {
            let d_idx = (x + y) % (n * 2);
            let a_idx = (n + x - y) % (n * 2);
            if !xset[x] && !dset[d_idx] && !aset[a_idx] {
                lines.push(x);
                xset[x] = true;
                dset[d_idx] = true;
                aset[a_idx] = true;
                Self::place_queesn(n, lines, xset, dset, aset, ans);
                lines.pop();
                xset[x] = false;
                dset[d_idx] = false;
                aset[a_idx] = false;
            }
        }
    }

    fn build_lint(n: usize, pos: usize) -> String {
        (0..n).map(|i| if i == pos { 'Q' } else { '.' }).collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    fn check(input: i32, mut expect: Vec<Vec<String>>) {
        let mut exact = Solution::solve_n_queens(input);
        exact.sort();
        expect.sort();
        assert_eq!(exact, expect);
    }

    #[test]
    fn test_4() {
        check(
            4,
            vec![
                vec![
                    ".Q..".to_owned(),
                    "...Q".to_owned(),
                    "Q...".to_owned(),
                    "..Q.".to_owned(),
                ],
                vec![
                    "..Q.".to_owned(),
                    "Q...".to_owned(),
                    "...Q".to_owned(),
                    ".Q..".to_owned(),
                ],
            ],
        );
    }

    #[test]
    fn test_empty() {
        check(0, Vec::new());
        check(-1, Vec::new());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::solve_n_queens(8));
    }
}
