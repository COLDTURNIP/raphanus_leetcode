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

impl Solution {
    pub fn largest_time_from_digits(a: Vec<i32>) -> String {
        // Permutation solution:
        // Heap's algorithm. Refer to component::permutation
        let mut data = a;
        let swap_state = &mut [0; 4];
        let mut max_hour = -1;
        let mut max_minute = -1;
        'perm: loop {
            let hour = data[0] * 10 + data[1];
            let minute = data[2] * 10 + data[3];
            if hour < 24 && minute < 60 && hour * 60 + minute > max_hour * 60 + max_minute {
                max_hour = hour;
                max_minute = minute;
            }

            let mut i = 1;
            loop {
                if i >= swap_state.len() { break 'perm; }
                if swap_state[i] < i { break; }
                swap_state[i] = 0;
                i += 1;
            }
            data.swap(i, (i & 1) * swap_state[i]);
            swap_state[i] += 1;
        }
        if max_hour < 0 || max_minute < 0 {
            "".to_owned()
        } else {
            format!("{:02}:{:02}", max_hour, max_minute)
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::largest_time_from_digits(vec![1, 2, 3, 4]),
            "23:41".to_owned()
        );
        assert_eq!(
            Solution::largest_time_from_digits(vec![5, 5, 5, 5]),
            "".to_owned()
        );
        assert_eq!(
            Solution::largest_time_from_digits(vec![2, 0, 6, 6]),
            "06:26".to_owned()
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::largest_time_from_digits(vec![1, 2, 3, 4]));
    }
}
