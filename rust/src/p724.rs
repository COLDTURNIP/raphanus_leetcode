/*
Problem 724. Find Pivot Index
=============================

https://leetcode.com/problems/find-pivot-index/

Given an array of integers nums, write a method that returns the "pivot" index of this array.

We define the pivot index as the index where the sum of all the numbers to the left of the index is
equal to the sum of all the numbers to the right of the index.

If no such index exists, we should return -1. If there are multiple pivot indexes, you should
return the left-most pivot index.

Example 1:

    Input: nums = [1,7,3,6,5,6]
    Output: 3
    Explanation:
    The sum of the numbers to the left of index 3 (nums[3] = 6) is equal to the sum of numbers to the right of index 3.
    Also, 3 is the first index where this occurs.

Example 2:

    Input: nums = [1,2,3]
    Output: -1
    Explanation:
    There is no index that satisfies the conditions in the problem statement.

Constraints:

    - The length of nums will be in the range [0, 10000].
    - Each element nums[i] will be an integer in the range [-1000, 1000].
*/

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        nums.into_iter()
            .enumerate()
            .scan(0i32, |left_sum, (i, n)| {
                let right_sum = sum - *left_sum - n;
                let pivot_idx = if right_sum == *left_sum { i as i32 } else { -1 };
                *left_sum += n;
                Some(pivot_idx)
            })
            .find(|&i| i >= 0)
            .unwrap_or(-1)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_found() {
        assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    }

    #[test]
    fn test_not_found() {
        assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::pivot_index(Vec::new()), -1);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::pivot_index(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    }
}
