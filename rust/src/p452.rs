/*
Problem 452. Minimum Number of Arrows to Burst Balloons
=======================================================

https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/

There are some spherical balloons spread in two-dimensional space. For each balloon, provided input
is the start and end coordinates of the horizontal diameter. Since it's horizontal, y-coordinates
don't matter, and hence the x-coordinates of start and end of the diameter suffice. The start is
always smaller than the end.

An arrow can be shot up exactly vertically from different points along the x-axis. A balloon with
xstart and xend bursts by an arrow shot at x if xstart ≤ x ≤ xend. There is no limit to the number
of arrows that can be shot. An arrow once shot keeps traveling up infinitely.

Given an array points where points[i] = [xstart, xend], return the minimum number of arrows that
must be shot to burst all balloons.

Example 1:

    Input: points = [[10,16],[2,8],[1,6],[7,12]]
    Output: 2
    Explanation: One way is to shoot one arrow for example at x = 6 (bursting the balloons [2,8]
                 and [1,6]) and another arrow at x = 11 (bursting the other two balloons).

Example 2:

    Input: points = [[1,2],[3,4],[5,6],[7,8]]
    Output: 4

Example 3:

    Input: points = [[1,2],[2,3],[3,4],[4,5]]
    Output: 2

Example 4:

    Input: points = [[1,2]]
    Output: 1

Example 5:

    Input: points = [[2,3],[2,3]]
    Output: 1

Constraints:

    - 0 <= points.length <= 10^4
    - points[i].length == 2
    - -2^31 <= xstart < xend <= 2^31 - 1
*/

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_unstable_by_key(|p| p[1]);
        let mut pt_iter = points.into_iter();
        let mut cnt = 0;
        if let Some(first_rng) = pt_iter.next() {
            cnt = 1;
            let mut bound = first_rng[1];
            for rng in pt_iter {
                if rng[0] > bound {
                    cnt += 1;
                    bound = rng[1];
                }
            }
        }
        cnt
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]),
            2
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]),
            4
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]),
            2
        );
    }

    #[test]
    fn test_case4() {
        assert_eq!(Solution::find_min_arrow_shots(vec![vec![1, 2]]), 1);
    }

    #[test]
    fn test_case5() {
        assert_eq!(
            Solution::find_min_arrow_shots(vec![vec![2, 3], vec![2, 3]]),
            1
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::find_min_arrow_shots(Vec::new()), 0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]);
        });
    }
}
