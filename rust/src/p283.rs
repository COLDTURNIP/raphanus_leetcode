/*
Problem 283. Move Zeroes
========================

https://leetcode.com/problems/move-zeroes/

Given an array nums, write a function to move all 0's to the end of it while maintaining the
relative order of the non-zero elements.

Example:

    Input: [0,1,0,3,12]
    Output: [1,3,12,0,0]

Note:

    - You must do this in-place without making a copy of the array.
    - Minimize the total number of operations.
*/

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let len = nums.len();
        let mut last_move = None;
        for i in 0..len {
            if nums[i] != 0 {
                continue;
            }
            let start_search = last_move.unwrap_or(i) + 1;
            if let Some(j) = (start_search..len).find(|&j| nums[j] != 0) {
                nums.swap(i, j);
                last_move = Some(j);
            } else {
                // all the rest items are zeros
                break;
            }
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    fn check(mut input: Vec<i32>, expect: Vec<i32>) {
        Solution::move_zeroes(&mut input);
        assert_eq!(input, expect);
    }

    #[test]
    fn test_move() {
        check(vec![0, 1, 0, 3, 12], vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn test_no_move() {
        check(vec![1, 3, 12, 0, 0], vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn test_empty() {
        check(Vec::new(), Vec::new());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::move_zeroes(&mut vec![0, 1, 0, 3, 12]));
    }
}
