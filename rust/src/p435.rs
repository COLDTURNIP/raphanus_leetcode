/*
Problem 435. Non-overlapping Intervals
======================================

https://leetcode.com/problems/non-overlapping-intervals/

Given a collection of intervals, find the minimum number of intervals you need to remove to make
the rest of the intervals non-overlapping.

Example 1:

    Input: [[1,2],[2,3],[3,4],[1,3]]
    Output: 1
    Explanation: [1,3] can be removed and the rest of intervals are non-overlapping.

Example 2:

    Input: [[1,2],[1,2],[1,2]]
    Output: 2
    Explanation: You need to remove two [1,2] to make the rest of intervals non-overlapping.

Example 3:

    Input: [[1,2],[2,3]]
    Output: 0
    Explanation: You don't need to remove any of the intervals since they're already non-overlapping.

Note:

    - You may assume the interval's end point is always bigger than its start point.
    - Intervals like [1,2] and [2,3] have borders "touching" but they don't overlap each other.
*/

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_unstable();
        let mut itv_iter = intervals.into_iter();
        if let Some(first) = itv_iter.next() {
            let mut rm_cnt = 0;
            let mut bound = first[1];
            for itv in itv_iter {
                if bound > itv[0] {
                    rm_cnt += 1;
                    bound = bound.min(itv[1]);
                } else {
                    bound = itv[1];
                }
            }
            rm_cnt
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
    fn test_case1() {
        assert_eq!(
            Solution::erase_overlap_intervals(
                vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3],]
            ),
            1
        )
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2],]),
            2
        )
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3],]),
            0
        )
    }

    #[test]
    fn test_case4() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![-100, -2],
                vec![5, 7]
            ]),
            0
        )
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::erase_overlap_intervals(Vec::new()), 0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]);
        });
    }
}
