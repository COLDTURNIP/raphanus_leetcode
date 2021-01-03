/*
Problem 539. Minimum Time Difference
====================================

Given a list of 24-hour clock time points in "HH:MM" format, return the minimum minutes difference
between any two time-points in the list.

Example 1:

    Input: timePoints = ["23:59","00:00"]
    Output: 1

Example 2:

    Input: timePoints = ["00:00","23:59","00:00"]
    Output: 0

Constraints:

    - 2 <= timePoints <= 2 * 104
    - timePoints[i] is in the format "HH:MM".
*/

impl Solution {
    fn parse_time_str_to_min(time_str: &str) -> i32 {
        let hour: i32 = time_str[0..2].parse().unwrap();
        let minute: i32 = time_str[3..5].parse().unwrap();
        hour * 60 + minute
    }

    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        if time_points.len() < 2 {
            return 0;
        }
        let mut time_points: Vec<i32> = time_points
            .into_iter()
            .map(|ts| Self::parse_time_str_to_min(&ts))
            .collect();
        time_points.sort_unstable();
        let mut ans = 24 * 60 - time_points.last().unwrap() + time_points.first().unwrap();
        for window in time_points.windows(2) {
            ans = ans.min(window[1] - window[0]);
            if ans == 0 {
                break;
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
        assert_eq!(
            Solution::find_min_difference(
                ["23:59", "00:00"].iter().map(|s| s.to_string()).collect()
            ),
            1
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::find_min_difference(
                ["00:00", "23:59", "00:00"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            ),
            0
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::find_min_difference(
                ["00:00", "23:59", "00:00"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            )
        });
    }
}
