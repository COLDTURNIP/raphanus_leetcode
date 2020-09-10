/*
Problem 62. Unique Paths
========================

https://leetcode.com/problems/unique-paths/

A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).

The robot can only move either down or right at any point in time. The robot is trying to reach the
bottom-right corner of the grid (marked 'Finish' in the diagram below).

How many possible unique paths are there?

https://assets.leetcode.com/uploads/2018/10/22/robot_maze.png
Above is a 7 x 3 grid. How many possible unique paths are there?

Example 1:

    Input: m = 3, n = 2
    Output: 3
    Explanation:
    From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
    1. Right -> Right -> Down
    2. Right -> Down -> Right
    3. Down -> Right -> Right

Example 2:

    Input: m = 7, n = 3
    Output: 28

Constraints:

    1 <= m, n <= 100
    It's guaranteed that the answer will be less than or equal to 2 * 10 ^ 9.
*/

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        // C(a, b) = C(m-1+n-1, n-1)
        let a = (m - 1 + n - 1) as u64;
        let b = (m - 1).max(n - 1) as u64;
        // calculate C(a, b) from C(b, b)
        (b + 1..=a).map(|an| (an, an - b)).fold(1, |acc, (an, bn)| {
            let (ran, rbn) = Self::reduction_fraction(an, bn);
            acc / rbn * ran
        }) as i32
    }

    fn reduction_fraction(a: u64, b: u64) -> (u64, u64) {
        let (mut m, mut n) = if a > b { (a, b) } else { (b, a) };
        while n != 0 {
            std::mem::swap(&mut m, &mut n);
            n %= m;
        }
        (a / m, b / m)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    #[test]
    fn test_3_2() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
    }

    #[test]
    fn test_7_3() {
        assert_eq!(Solution::unique_paths(7, 3), 28);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::unique_paths(12, 10));
    }
}
