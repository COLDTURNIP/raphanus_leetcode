/*
Problem 120. Triangle
=====================

https://leetcode.com/problems/triangle/

Given a triangle, find the minimum path sum from top to bottom. Each step you may move to adjacent
numbers on the row below.

For example, given the following triangle

    [
         [2],
        [3,4],
       [6,5,7],
      [4,1,8,3]
    ]

The minimum path sum from top to bottom is 11 (i.e., 2 + 3 + 5 + 1 = 11).

Note:

    Bonus point if you are able to do this using only O(n) extra space, where n is the total number
    of rows in the triangle.
*/

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut row_iter = triangle.into_iter().rev();
        if let Some(ref mut buf) = row_iter.next() {
            for row in row_iter {
                for (i, prev) in row.into_iter().enumerate() {
                    buf[i] = (prev + buf[i]).min(prev + buf[i + 1]);
                }
            }
            buf[0]
        } else {
            0
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_min() {
        assert_eq!(
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![5, 6, 7], vec![4, 1, 8, 3],],),
            11
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::minimum_total(Vec::new()), 0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![5, 6, 7], vec![4, 1, 8, 3]]);
        });
    }
}
