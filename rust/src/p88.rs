/*
Problem 88. Merge Sorted Array
==============================

https://leetcode.com/problems/merge-sorted-array/

Given two sorted integer arrays nums1 and nums2, merge nums2 into nums1 as one sorted array.

Note:

    - The number of elements initialized in nums1 and nums2 are m and n respectively.
    - You may assume that nums1 has enough space (size that is equal to m + n) to hold additional
      elements from nums2.

Example:

    Input:
    nums1 = [1,2,3,0,0,0], m = 3
    nums2 = [2,5,6],       n = 3

    Output: [1,2,2,3,5,6]


Constraints:

    -10^9 <= nums1[i], nums2[i] <= 10^9
    nums1.length == m + n
    nums2.length == n
*/

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        assert_eq!(nums2.len(), n as usize);
        if n == 0 {
            return;
        }
        let mut buf_idx = m as usize;
        let mut tail2_idx = n as usize - 1;
        for tail_idx in (0..nums1.len()).rev() {
            match (buf_idx.checked_sub(1).map(|i| nums1[i]), nums2[tail2_idx]) {
                (Some(n1), n2) if n1 > n2 => {
                    nums1[tail_idx] = n1;
                    nums1[buf_idx - 1] = 0;
                    buf_idx -= 1;
                }
                (_, n2) => {
                    nums1[tail_idx] = n2;
                    if tail2_idx == 0 {
                        return;
                    } else {
                        tail2_idx -= 1;
                    }
                }
            }
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    fn check(mut nums1: Vec<i32>, m: i32, mut nums2: Vec<i32>, n: i32, expect: Vec<i32>) {
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, expect);
    }

    #[test]
    fn test_merge_123_256() {
        check(
            vec![1, 2, 3, 0, 0, 0],
            3,
            vec![2, 5, 6],
            3,
            vec![1, 2, 2, 3, 5, 6],
        );
    }

    #[test]
    fn test_merge_empty() {
        check(vec![0, 0, 0], 0, vec![2, 5, 6], 3, vec![2, 5, 6]);
        check(vec![1, 2, 3], 3, Vec::new(), 0, vec![1, 2, 3]);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::merge(&mut vec![1, 2, 3, 0, 0, 0], 3, &mut vec![2, 5, 6], 3));
    }
}
