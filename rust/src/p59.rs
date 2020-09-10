/*
Problem 59. Spiral Matrix II
============================

https://leetcode.com/problems/spiral-matrix-ii/

Given a positive integer n, generate a square matrix filled with elements from 1 to n2 in spiral
order.

Example:

    Input: 3
    Output:
    [
     [ 1, 2, 3 ],
     [ 8, 9, 4 ],
     [ 7, 6, 5 ]
    ]
*/

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        if n <= 0 {
            return Vec::new();
        }
        let len = n as usize;
        let mut out = vec![vec![0; len]; len];
        enum Direction {
            Right(usize),
            Down(usize),
            Left(usize),
            Up(usize),
        }
        let mut cur_len = len;
        let mut pos = (0, 0);
        let mut dir = Direction::Right(len - 1);
        out[0][0] = 1;
        let mut cur_val = 2;
        while cur_val <= n * n {
            match dir {
                Direction::Right(n) => {
                    (1..=n).for_each(|i| {
                        out[pos.1][pos.0 + i] = cur_val;
                        cur_val += 1;
                    });
                    pos.0 += n;
                    dir = Direction::Down(cur_len - 1)
                }
                Direction::Down(n) => {
                    (1..=n).for_each(|i| {
                        out[pos.1 + i][pos.0] = cur_val;
                        cur_val += 1;
                    });
                    pos.1 += n;
                    dir = Direction::Left(cur_len - 1)
                }
                Direction::Left(n) => {
                    (1..=n).for_each(|i| {
                        out[pos.1][pos.0 - i] = cur_val;
                        cur_val += 1;
                    });
                    pos.0 -= n;
                    dir = Direction::Up(cur_len - 2)
                }
                Direction::Up(n) => {
                    (1..=n).for_each(|i| {
                        out[pos.1 - i][pos.0] = cur_val;
                        cur_val += 1;
                    });
                    pos.1 -= n;
                    cur_len -= 2;
                    dir = Direction::Right(cur_len)
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
    fn test_empty() {
        assert_eq!(Solution::generate_matrix(0), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_1x1() {
        assert_eq!(Solution::generate_matrix(1), vec![vec![1]]);
    }

    #[test]
    fn test_4x4() {
        assert_eq!(
            Solution::generate_matrix(4),
            vec![
                vec![1, 2, 3, 4],
                vec![12, 13, 14, 5],
                vec![11, 16, 15, 6],
                vec![10, 9, 8, 7],
            ]
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::generate_matrix(10));
    }
}
