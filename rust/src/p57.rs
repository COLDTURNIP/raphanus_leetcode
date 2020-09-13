/*
Problem 57. Insert Interval
===========================

https://leetcode.com/problems/insert-interval/

Given a set of non-overlapping intervals, insert a new interval into the intervals (merge if
necessary).

You may assume that the intervals were initially sorted according to their start times.

Example 1:

    Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
    Output: [[1,5],[6,9]]

Example 2:

    Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
    Output: [[1,2],[3,10],[12,16]]
    Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].

NOTE: input types have been changed on April 15, 2019. Please reset to default code definition to
get new method signature.
*/

use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut prev_cmp = Ordering::Less;
        let mut merged = new_interval;
        for itv in intervals.into_iter() {
            let cur_cmp = Self::cmp_interval(&merged, &itv);
            match (prev_cmp, cur_cmp) {
                (Less, Greater) | (Equal, Greater) => {
                    result.push(merged.clone());
                    result.push(itv);
                }
                (Less, Equal) | (Equal, Equal) => Self::merge_interval(&mut merged, &itv),
                (Less, Less) | (Greater, Greater) => result.push(itv),
                _ => unreachable!(),
            }
            prev_cmp = cur_cmp;
        }
        if prev_cmp != Greater {
            result.push(merged);
        }
        result
    }

    fn cmp_interval(this: &[i32], other: &[i32]) -> Ordering {
        if other[1] < this[0] {
            Ordering::Less
        } else if other[0] > this[1] {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    fn merge_interval(this: &mut [i32], other: &[i32]) {
        this[0] = this[0].min(other[0]);
        this[1] = this[1].max(other[1]);
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_insert_non_overlap() {
        assert_eq!(
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
    }

    #[test]
    fn test_insert_overlap() {
        assert_eq!(
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16],
                ],
                vec![4, 8],
            );
        });
    }
}
