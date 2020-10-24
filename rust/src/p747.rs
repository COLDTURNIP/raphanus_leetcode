/*
Problem 747. Largest Number At Least Twice of Others
====================================================

https://leetcode.com/problems/largest-number-at-least-twice-of-others/

In a given integer array nums, there is always exactly one largest element.

Find whether the largest element in the array is at least twice as much as every other number in
the array.

If it is, return the index of the largest element, otherwise return -1.

Example 1:

    Input: nums = [3, 6, 1, 0]
    Output: 1
    Explanation: 6 is the largest integer, and for every other number in the array x,
    6 is more than twice as big as x.  The index of value 6 is 1, so we return 1.

Example 2:

    Input: nums = [1, 2, 3, 4]
    Output: -1
    Explanation: 4 isn't at least as big as twice the value of 3, so we return -1.

Note:

    - nums will have a length in the range [1, 50].
    - Every nums[i] will be an integer in the range [0, 99].
*/

impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut iter = nums.into_iter().enumerate();
        let mut max = -1;
        let mut max_idx = 0;
        let mut second = -1;
        if let Some((_, first)) = iter.next() {
            max = first;
            for (i, n) in iter {
                if n > max {
                    second = max;
                    max = n;
                    max_idx = i;
                } else {
                    second = second.max(n);
                }
            }
        }
        if max > 0 && max >= second * 2 {
            max_idx as i32
        } else {
            -1
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_3610() {
        assert_eq!(Solution::dominant_index(vec![3, 6, 1, 0]), 1);
    }

    #[test]
    fn test_1234() {
        assert_eq!(Solution::dominant_index(vec![1, 2, 3, 4]), -1);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::dominant_index(vec![9]), 0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::dominant_index(vec![0, 2, 3, 4, 6, 8, 9]));
    }
}
