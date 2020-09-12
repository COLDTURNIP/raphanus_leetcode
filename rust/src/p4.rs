/*
Problem 4. Median of Two Sorted Arrays
======================================

Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two
sorted arrays.

Follow up: The overall run time complexity should be O(log (m+n)).

Example 1:

    Input: nums1 = [1,3], nums2 = [2]
    Output: 2.00000
    Explanation: merged array = [1,2,3] and median is 2.

Example 2:

    Input: nums1 = [1,2], nums2 = [3,4]
    Output: 2.50000
    Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.

Example 3:

    Input: nums1 = [0,0], nums2 = [0,0]
    Output: 0.00000

Example 4:

    Input: nums1 = [], nums2 = [1]
    Output: 1.00000

Example 5:

    Input: nums1 = [2], nums2 = []
    Output: 2.00000

Constraints:

    nums1.length == m
    nums2.length == n
    0 <= m <= 1000
    0 <= n <= 1000
    1 <= m + n <= 2000
    -10^6 <= nums1[i], nums2[i] <= 10^6
*/

use std::cmp::Ordering;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (n1, n2, m, n) = {
            let len1 = nums1.len();
            let len2 = nums2.len();
            if len1 < len2 {
                (nums1, nums2, len1, len2)
            } else {
                (nums2, nums1, len2, len1)
            }
        };
        let total = m + n;
        let mid = (total + 1) / 2;
        let (mut lb, mut rb) = (0, m);

        #[derive(Debug)]
        struct Med {
            l1: Option<i32>,
            l2: Option<i32>,
            r1: Option<i32>,
            r2: Option<i32>,
        }
        let get_mids = |i1: usize, i2: usize| Med {
            l1: i1.checked_sub(1).and_then(|i| n1.get(i)).cloned(),
            l2: i2.checked_sub(1).and_then(|i| n2.get(i)).cloned(),
            r1: n1.get(i1).cloned(),
            r2: n2.get(i2).cloned(),
        };

        let meds = loop {
            let mid1 = (lb + rb) / 2;
            let mid2 = mid - mid1;
            let meds = get_mids(mid1, mid2);
            if lb >= rb {
                break meds;
            }
            match (
                Self::option_cmp(meds.l1, meds.r2),
                Self::option_cmp(meds.l2, meds.r1),
            ) {
                (Some(Ordering::Greater), _) => rb = mid1,
                (_, Some(Ordering::Greater)) => lb = mid1 + 1,
                _ => break meds,
            }
        };
        let lm = Self::option_max(meds.l1, meds.l2);
        if total % 2 == 1 {
            lm as f64
        } else {
            let rm = Self::option_min(meds.r1, meds.r2);
            (lm + rm) as f64 / 2.0
        }
    }

    fn option_max(on1: Option<i32>, on2: Option<i32>) -> i32 {
        on1.unwrap_or(std::i32::MIN)
            .max(on2.unwrap_or(std::i32::MIN))
    }

    fn option_min(on1: Option<i32>, on2: Option<i32>) -> i32 {
        on1.unwrap_or(std::i32::MAX)
            .min(on2.unwrap_or(std::i32::MAX))
    }

    fn option_cmp(on1: Option<i32>, on2: Option<i32>) -> Option<Ordering> {
        on1.and_then(|n1| on2.map(|n2| n1.cmp(&n2)))
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    fn check(n1: Vec<i32>, n2: Vec<i32>, expect: f64) {
        assert!((Solution::find_median_sorted_arrays(n1, n2) - expect).abs() < std::f64::EPSILON);
    }

    #[test]
    fn test_med_2x1() {
        check(vec![1, 3], vec![2], 2.0);
    }

    #[test]
    fn test_med_2x2() {
        check(vec![1, 2], vec![3, 4], 2.5);
    }

    #[test]
    fn test_med_zeros() {
        check(vec![0, 0], vec![0, 0], 0.0);
    }

    #[test]
    fn test_med_empty() {
        check(Vec::new(), vec![1], 1.0);
        check(vec![2], Vec::new(), 2.0);
    }

    #[test]
    fn test_overlap() {
        check(vec![1, 3, 3, 4, 5, 9], vec![2, 7, 8, 9], 4.5);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::find_median_sorted_arrays(vec![1, 3, 3, 4, 5, 9], vec![2, 7, 8, 9]));
    }
}
