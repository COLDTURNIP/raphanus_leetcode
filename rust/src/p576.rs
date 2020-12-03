/*
Problem 576. Out of Boundary Paths
==================================

https://leetcode.com/problems/out-of-boundary-paths/

There is an m by n grid with a ball. Given the start coordinate (i,j) of the ball, you can move the
ball to adjacent cell or cross the grid boundary in four directions (up, down, left, right).
However, you can at most move N times. Find out the number of paths to move the ball out of grid
boundary. The answer may be very large, return it after mod 10^9 + 7.

Example 1:

    Input: m = 2, n = 2, N = 2, i = 0, j = 0
    Output: 6
    Explanation:
    https://assets.leetcode.com/uploads/2018/10/13/out_of_boundary_paths_1.png

Example 2:

    Input: m = 1, n = 3, N = 3, i = 0, j = 1
    Output: 12
    Explanation:
    https://assets.leetcode.com/uploads/2018/10/12/out_of_boundary_paths_2.png

Note:

    - Once you move the ball out of boundary, you cannot move it back.
    - The length and height of the grid is in range [1,50].
    - N is in range [0,50].
*/

impl Solution {
    pub fn find_paths(m: i32, n: i32, steps: i32, i: i32, j: i32) -> i32 {
        if steps <= 0 {
            return 0;
        }

        let cnt_add = |c: &mut i32, n: i32| {
            *c = (*c + n) % 1_000_000_007;
        };

        let m = m as usize;
        let n = n as usize;
        let mut count: Vec<Vec<i32>> = vec![vec![0; n]; m];
        let mut last_count = count.clone();
        let mut ans = 0;
        count[i as usize][j as usize] = 1;
        for _step in 1..=steps {
            std::mem::swap(&mut count, &mut last_count);
            for i in 0..m {
                for j in 0..n {
                    let last_c = last_count[i][j];
                    count[i][j] = 0;
                    if i == 0 {
                        cnt_add(&mut ans, last_c);
                    } else {
                        cnt_add(&mut count[i][j], last_count[i - 1][j]);
                    };
                    if i == m - 1 {
                        cnt_add(&mut ans, last_c);
                    } else {
                        cnt_add(&mut count[i][j], last_count[i + 1][j]);
                    };
                    if j == 0 {
                        cnt_add(&mut ans, last_c);
                    } else {
                        cnt_add(&mut count[i][j], last_count[i][j - 1]);
                    };
                    if j == n - 1 {
                        cnt_add(&mut ans, last_c);
                    } else {
                        cnt_add(&mut count[i][j], last_count[i][j + 1]);
                    };
                }
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
    fn test_case1() {
        assert_eq!(Solution::find_paths(2, 2, 2, 0, 0), 6);
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::find_paths(1, 3, 3, 0, 1), 12);
    }

    #[test]
    fn test_zero() {
        assert_eq!(Solution::find_paths(2, 3, 0, 0, 0), 0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::find_paths(2, 3, 5, 0, 1));
    }
}
