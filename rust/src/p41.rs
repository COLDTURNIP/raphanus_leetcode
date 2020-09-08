/*
Problem 41. First Missing Positive
==================================

https://leetcode.com/problems/first-missing-positive/

Given an unsorted integer array, find the smallest missing positive integer.

Example 1:

    Input: [1,2,0]
    Output: 3

Example 2:

    Input: [3,4,-1,1]
    Output: 2

Example 3:

    Input: [7,8,9,11,12]
    Output: 1

Follow up:

    Your algorithm should run in O(n) time and uses constant extra space.
*/

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();
        for i in 0..len {
            let mut cv = nums[i];
            while cv > 0 && cv <= len as i32 && cv - 1 != i as i32 {
                if nums[i] == nums[cv as usize - 1] {
                    break;
                }
                nums.swap(i, cv as usize - 1);
                cv = nums[i];
            }
        }
        (0..len).find(|&i| nums[i] - 1 != i as i32).unwrap_or(len) as i32 + 1
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    #[test]
    fn test_start_1() {
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
    }

    #[test]
    fn test_double_1() {
        assert_eq!(Solution::first_missing_positive(vec![1, 1]), 2);
    }

    #[test]
    fn test_start_1_discontinue() {
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
    }

    #[test]
    fn test_start_n() {
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::first_missing_positive(Vec::new()), 1);
    }

    #[test]
    fn test_continuous() {
        assert_eq!(Solution::first_missing_positive(vec![1, 6, 4, 3, 5, 2]), 7);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::first_missing_positive(vec![1, 2, 4, 7, 8, 9, 10]));
    }
}
