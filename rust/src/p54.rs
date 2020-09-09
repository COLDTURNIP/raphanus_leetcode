/*
Problem 54. Spiral Matrix
=========================

https://leetcode.com/problems/spiral-matrix/

Given a matrix of m x n elements (m rows, n columns), return all elements of the matrix in spiral
order.

Example 1:

    Input:
    [
     [ 1, 2, 3 ],
     [ 4, 5, 6 ],
     [ 7, 8, 9 ]
    ]
    Output: [1,2,3,6,9,8,7,4,5]

Example 2:

    Input:
    [
      [1, 2, 3, 4],
      [5, 6, 7, 8],
      [9,10,11,12]
    ]
    Output: [1,2,3,4,8,12,11,10,9,5,6,7]
*/

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() || matrix.iter().any(|row| row.is_empty()) {
            return Vec::new();
        }
        enum Direction {
            Right(usize),
            Down(usize),
            Left(usize),
            Up(usize),
        }
        let (max_row, max_col) = (matrix.len(), matrix[0].len());
        let (mut cur_max_row, mut cur_max_col) = (max_row, max_col);
        let mut pos = (0, 0);
        let mut dir = Direction::Right(max_col - 1);
        let mut out = Vec::with_capacity(max_col * max_row);
        out.push(matrix[0][0]);
        while out.len() < out.capacity() {
            match dir {
                Direction::Right(n) => {
                    (1..=n).for_each(|i| out.push(matrix[pos.1][pos.0 + i]));
                    pos.0 += n;
                    dir = Direction::Down(cur_max_row - 1)
                }
                Direction::Down(n) => {
                    (1..=n).for_each(|i| out.push(matrix[pos.1 + i][pos.0]));
                    pos.1 += n;
                    dir = Direction::Left(cur_max_col - 1)
                }
                Direction::Left(n) => {
                    (1..=n).for_each(|i| out.push(matrix[pos.1][pos.0 - i]));
                    pos.0 -= n;
                    dir = Direction::Up(cur_max_row - 2)
                }
                Direction::Up(n) => {
                    (1..=n).for_each(|i| out.push(matrix[pos.1 - i][pos.0]));
                    pos.1 -= n;
                    cur_max_col -= 2;
                    cur_max_row -= 2;
                    dir = Direction::Right(cur_max_col)
                }
            };
        }
        out
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
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9],]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
    }

    #[test]
    fn test_3x4() {
        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
            ]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::spiral_order(vec![vec![1],]), vec![1]);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::spiral_order(Vec::new()), Vec::new());
        assert_eq!(
            Solution::spiral_order(vec![Vec::new(), Vec::new()]),
            Vec::new()
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
            ])
        });
    }
}
