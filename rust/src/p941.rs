/*
Problem 941. Valid Mountain Array
=================================

https://leetcode.com/problems/valid-mountain-array/

Given an array A of integers, return true if and only if it is a valid mountain array.

Recall that A is a mountain array if and only if:

    - A.length >= 3
    - There exists some i with 0 < i < A.length - 1 such that:
    - A[0] < A[1] < ... A[i-1] < A[i]
    - A[i] > A[i+1] > ... > A[A.length - 1]

Example 1:

    Input: [2,1]
    Output: false

Example 2:

    Input: [3,5,5]
    Output: false

Example 3:

    Input: [0,3,2,1]
    Output: true
*/

impl Solution {
    pub fn valid_mountain_array(a: Vec<i32>) -> bool {
        let len = a.len();
        if len < 3 {
            false
        } else {
            let (mut lacme, mut racme) = (0, len - 1);
            while lacme < len - 1 && a[lacme] < a[lacme + 1] {
                lacme += 1;
            }
            while racme > 0 && a[racme] < a[racme - 1] {
                racme -= 1;
            }
            lacme > 0 && racme < len - 1 && lacme == racme
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_valid() {
        assert!(Solution::valid_mountain_array(vec![0, 3, 2, 1]));
    }

    #[test]
    fn test_invalid() {
        assert!(!Solution::valid_mountain_array(vec![3, 5, 5]));
        assert!(!Solution::valid_mountain_array(vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9
        ]));
    }

    #[test]
    fn test_too_short() {
        assert!(!Solution::valid_mountain_array(vec![2, 1]));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::valid_mountain_array(vec![0, 2, 3, 4, 5, 2, 1, 0]));
    }
}
