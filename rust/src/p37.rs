/*
Problem 37. Sudoku Solver
=========================

https://leetcode.com/problems/sudoku-solver/

Write a program to solve a Sudoku puzzle by filling the empty cells.

A sudoku solution must satisfy all of the following rules:

    1. Each of the digits 1-9 must occur exactly once in each row.
    2. Each of the digits 1-9 must occur exactly once in each column.
    3. Each of the the digits 1-9 must occur exactly once in each of the 9 3x3 sub-boxes of the
       grid.

Empty cells are indicated by the character '.'.

    https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png

A sudoku puzzle...

    https://upload.wikimedia.org/wikipedia/commons/thumb/3/31/Sudoku-by-L2G-20050714_solution.svg/250px-Sudoku-by-L2G-20050714_solution.svg.png

...and its solution numbers marked in red.

Note:

    - The given board contain only digits 1-9 and the character '.'.
    - You may assume that the given Sudoku puzzle will have a single unique solution.
    - The given board size is always 9x9.
*/

use std::iter::repeat_with;

impl Solution {
    const INT_TO_CHAR: &'static [char] = &['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    const BSIZE: usize = 9;
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut row_filled: Vec<Vec<bool>> = repeat_with(|| vec![false; 9]).take(Self::BSIZE).collect();
        let mut col_filled: Vec<Vec<bool>> = repeat_with(|| vec![false; 9]).take(Self::BSIZE).collect();
        let mut box_filled: Vec<Vec<bool>> = repeat_with(|| vec![false; 9]).take(Self::BSIZE).collect();
        let mut to_fill = Vec::<(usize, usize)>::new();
        for (ri, row) in row_filled.iter_mut().enumerate() {
            for (ci, col) in col_filled.iter_mut().enumerate() {
                let bi = (ri / 3) * 3 + (ci / 3);
                if let Some(val) = board[ri][ci].to_digit(10) {
                    let vi = val as usize - 1;
                    row[vi] = true;
                    col[vi] = true;
                    box_filled[bi][vi] = true;
                } else {
                    to_fill.push((ri, ci));
                }
            }
        }
        Self::solve_row(
            board,
            &to_fill,
            0,
            &mut row_filled,
            &mut col_filled,
            &mut box_filled,
        );
    }

    fn solve_row(
        board: &mut Vec<Vec<char>>,
        to_fill: &[(usize, usize)],
        cur_pos: usize,
        row_filled: &mut Vec<Vec<bool>>,
        col_filled: &mut Vec<Vec<bool>>,
        box_filled: &mut Vec<Vec<bool>>,
    ) -> bool {
        if cur_pos >= to_fill.len() {
            return true;
        }
        let (ri, ci) = to_fill[cur_pos];
        let bi = (ri / 3) * 3 + (ci / 3);
        for val in 1..=9 {
            let vi = val - 1;
            if row_filled[ri][vi] || col_filled[ci][vi] || box_filled[bi][vi] {
                continue;
            }
            row_filled[ri][vi] = true;
            col_filled[ci][vi] = true;
            box_filled[bi][vi] = true;
            board[ri][ci] = Self::INT_TO_CHAR[vi];
            if Self::solve_row(
                board,
                to_fill,
                cur_pos + 1,
                row_filled,
                col_filled,
                box_filled,
            ) {
                return true;
            }
            row_filled[ri][vi] = false;
            col_filled[ci][vi] = false;
            box_filled[bi][vi] = false;
        }
        false
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    fn check(input: Vec<&str>, expect: Vec<&str>) {
        let mut exact = input
            .into_iter()
            .map(|row| row.chars().collect())
            .collect::<Vec<Vec<char>>>();
        let expect = expect
            .into_iter()
            .map(|row| row.chars().collect())
            .collect::<Vec<Vec<char>>>();
        Solution::solve_sudoku(&mut exact);
        assert_eq!(exact, expect);
    }

    #[test]
    fn test_solve() {
        check(
            vec![
                &"53..7....",
                &"6..195...",
                &".98....6.",
                &"8...6...3",
                &"4..8.3..1",
                &"7...2...6",
                &".6....28.",
                &"...419..5",
                &"....8..79",
            ],
            vec![
                &"534678912",
                &"672195348",
                &"198342567",
                &"859761423",
                &"426853791",
                &"713924856",
                &"961537284",
                &"287419635",
                &"345286179",
            ],
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::solve_sudoku(&mut vec![
                "53..7....".chars().collect(),
                "6..195...".chars().collect(),
                ".98....6.".chars().collect(),
                "8...6...3".chars().collect(),
                "4..8.3..1".chars().collect(),
                "7...2...6".chars().collect(),
                ".6....28.".chars().collect(),
                "...419..5".chars().collect(),
                "....8..79".chars().collect(),
            ]);
        });
    }
}
