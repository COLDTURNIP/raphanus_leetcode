/*
Problem 75. Sort Colors
=======================

Given an array nums with n objects colored red, white, or blue, sort them in-place so that objects
of the same color are adjacent, with the colors in the order red, white, and blue.

Here, we will use the integers 0, 1, and 2 to represent the color red, white, and blue
respectively.

Follow up:

    - Could you solve this problem without using the library's sort function?
    - Could you come up with a one-pass algorithm using only O(1) constant space?


Example 1:

    Input: nums = [2,0,2,1,1,0]
    Output: [0,0,1,1,2,2]

Example 2:

    Input: nums = [2,0,1]
    Output: [0,1,2]

Example 3:

    Input: nums = [0]
    Output: [0]

Example 4:

    Input: nums = [1]
    Output: [1]

Constraints:

    n == nums.length
    1 <= n <= 300
    nums[i] is 0, 1, or 2.
*/

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.is_empty() {
            return;
        }

        // li: the most left index that value may not be 0
        // ii: the most right index that value may not be 2
        let (mut li, mut ri) = (0, nums.len() - 1);

        let mut i = 0;
        while i <= ri {
            match nums[i] {
                0 => {
                    nums.swap(li, i);
                    li += 1;
                    i += 1;
                }
                2 => {
                    nums.swap(ri, i);
                    if ri == 0 {
                        break;
                    } else {
                        ri -= 1;
                    }
                }
                _ => i += 1,
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
        Solution::sort_colors(&mut input);
        assert_eq!(input, expect);
    }

    #[test]
    fn test_sort6() {
        check(vec![2, 0, 2, 1, 1, 0], vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_sort201() {
        check(vec![2, 0, 1], vec![0, 1, 2]);
    }

    #[test]
    fn test_sort120() {
        check(vec![1, 2, 0], vec![0, 1, 2]);
    }

    #[test]
    fn test_single() {
        check(vec![0], vec![0]);
        check(vec![1], vec![1]);
        check(vec![2], vec![2]);
    }

    #[test]
    fn test_empty() {
        check(Vec::new(), Vec::new());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::sort_colors(&mut vec![2, 0, 2, 2, 1, 1, 0, 2, 0, 1, 2]);
        });
    }
}
