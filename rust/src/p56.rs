/*
Problem 56. Merge Intervals
===========================

https://leetcode.com/problems/merge-intervals/

Given a collection of intervals, merge all overlapping intervals.

Example 1:

    Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
    Output: [[1,6],[8,10],[15,18]]
    Explanation: Since intervals [1,3] and [2,6] overlaps, merge them into [1,6].

Example 2:

    Input: intervals = [[1,4],[4,5]]
    Output: [[1,5]]
    Explanation: Intervals [1,4] and [4,5] are considered overlapping.

NOTE: input types have been changed on April 15, 2019. Please reset to default code definition to
get new method signature.

Constraints:

    intervals[i][0] <= intervals[i][1]
*/

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return intervals;
        }
        let mut intervals = intervals;
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));
        let (mut li, mut ri) = (intervals[0][0], intervals[0][1]);
        let mut mut_cur = 0;
        for cur in 1..intervals.len() {
            let (next_li, next_ri) = (intervals[cur][0], intervals[cur][1]);
            if next_li <= ri {
                li = li.min(next_li);
                ri = ri.max(next_ri);
            } else {
                intervals[mut_cur][0] = li;
                intervals[mut_cur][1] = ri;
                li = next_li;
                ri = next_ri;
                mut_cur += 1;
            }
        }
        intervals[mut_cur][0] = li;
        intervals[mut_cur][1] = ri;
        intervals.truncate(mut_cur + 1);
        intervals
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    #[test]
    fn test_merge_overlap() {
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }

    #[test]
    fn test_merge_non_overlap() {
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![4, 5]]),
            vec![vec![1, 5]]
        );
    }

    #[test]
    fn test_merge_empty() {
        assert_eq!(Solution::merge(Vec::new()), Vec::<Vec<i32>>::new());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]));
    }
}
