/*
Problem 1124. Longest Well-Performing Interval
==============================================

https://leetcode.com/problems/longest-well-performing-interval/

We are given hours, a list of the number of hours worked per day for a given employee.

A day is considered to be a tiring day if and only if the number of hours worked is (strictly)
greater than 8.

A well-performing interval is an interval of days for which the number of tiring days is strictly
larger than the number of non-tiring days.

Return the length of the longest well-performing interval.

Example 1:

    Input: hours = [9,9,6,0,6,6,9]
    Output: 3
    Explanation: The longest well-performing interval is [9,9,6].

Constraints:

    - 1 <= hours.length <= 10000
    - 0 <= hours[i] <= 16
*/

use std::collections::HashMap;

impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let mut max_len = 0;
        let mut seen_sum_pos = HashMap::new();
        let mut interval_sum = 0;
        for (day, hour) in hours.into_iter().enumerate() {
            let day = day as i32;
            if hour > 8 {
                interval_sum += 1;
            } else {
                interval_sum -= 1;
            }
            seen_sum_pos.entry(interval_sum).or_insert(day as i32);
            if interval_sum > 0 {
                // notice that we can take whole subarray here, max_len.max(day + 1) == day + 1
                max_len = day + 1;
            } else if let Some(&past_interval_begin) = seen_sum_pos.get(&(interval_sum - 1)) {
                max_len = max_len.max(day - past_interval_begin);
            }
        }
        max_len
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(Solution::longest_wpi(vec![9, 9, 6, 0, 6, 6, 9]), 3);
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::longest_wpi(vec![9, 9, 6, 0, 10, 6, 11, 9, 12, 15, 6, 9, 1, 2, 3]),
            15
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(Solution::longest_wpi(vec![6, 6, 9]), 1);
    }

    #[test]
    fn test_case4() {
        assert_eq!(Solution::longest_wpi(vec![9, 6, 6]), 1);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::longest_wpi(Vec::new()), 0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::longest_wpi(vec![9, 9, 6, 0, 6, 6, 9]));
    }
}
