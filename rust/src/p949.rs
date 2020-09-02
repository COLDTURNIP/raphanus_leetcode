/*
949. Largest Time for Given Digits
=======================

https://leetcode.com/problems/largest-time-for-given-digits/

Given an array of 4 digits, return the largest 24 hour time that can be made.

The smallest 24 hour time is 00:00, and the largest is 23:59.  Starting from 00:00, a time is
larger if more time has elapsed since midnight.

Return the answer as a string of length 5.  If no valid time can be made, return an empty string.

Example 1:

Input: [1,2,3,4]
Output: "23:41"
Example 2:

Input: [5,5,5,5]
Output: ""
*/

// TODO: implement permutation iterator

impl Solution {
    pub fn largest_time_from_digits(a: Vec<i32>) -> String {
        //let mut nums = a.iter().rev().map(|&n| NumItem(n, false)).collect::<Vec<_>>();
        let mut nums = a;
        nums.sort_unstable_by(|a, b| b.cmp(a));
        for i1 in 0..4 {
            for i2 in (0..4).filter(|&i| i != i1) {
                for i3 in (0..4).filter(|&i| i != i1 && i != i2) {
                    for i4 in (0..4).filter(|&i| i != i1 && i != i2 && i != i3) {
                        let hour = nums[i1] * 10 + nums[i2];
                        let minute = nums[i3] * 10 + nums[i4];
                        if hour < 24 && minute < 60 {
                            return format!("{:02}:{:02}", hour, minute)
                        }
                    }
                }
            }
        };
        "".to_owned()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::largest_time_from_digits(vec![1, 2, 3, 4]), "23:41".to_owned());
        assert_eq!(Solution::largest_time_from_digits(vec![5, 5, 5, 5]), "".to_owned());
        assert_eq!(Solution::largest_time_from_digits(vec![2, 0, 6, 6]), "06:26".to_owned());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::largest_time_from_digits(vec![1, 2, 3, 4]));
    }
}
