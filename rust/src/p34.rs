/*
34. Find First and Last Position of Element in Sorted Array
=======================================

https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/

Given an array of integers nums sorted in ascending order, find the starting and ending position of
a given target value.

Your algorithm's runtime complexity must be in the order of O(log n).

If the target is not found in the array, return [-1, -1].

Example 1:

    Input: nums = [5,7,7,8,8,10], target = 8
    Output: [3,4]

Example 2:

    Input: nums = [5,7,7,8,8,10], target = 6
    Output: [-1,-1]
*/

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::cmp::Ordering;

        if nums.is_empty() {
            return vec![-1, -1];
        }
        let mut search_low = 0usize;
        let mut search_high = nums.len();
        let mut search_mid = (search_low + search_high) / 2;
        if nums[search_low] > target || nums[search_high-1] < target {
            return vec![-1, -1];
        }
        while search_low < search_high {
            let mid = (search_low + search_high) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Equal => {
                    search_mid = mid;
                    break;
                },
                Ordering::Less => search_low = mid+1,
                Ordering::Greater => search_high = mid,
            }
        }
        if search_low >= search_high {
            return vec![-1, -1];
        }

        // search for lower bound
        let mut low = search_low;
        {
            let mut search_high = search_mid;
            while nums[low] != target && low < search_high {
                let mid = (low + search_high) / 2;
                if nums[mid] == target {
                    search_high = mid;
                } else {
                    low = mid+1;
                }
            }
        }
        // search for higher bound
        let mut high = search_high;
        {
            let mut search_low = search_mid;
            while nums[high-1] != target && search_low < high {
                let mid = (search_low + high) / 2;
                if nums[mid] == target {
                    search_low = mid+1;
                } else {
                    high = mid;
                }
            }
        }
        vec![low as i32, high as i32 -1]
    }
}

pub struct Solution;

mod internal {
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
        assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use super::Solution;

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::search_range(vec![5, 7, 7, 8, 8, 10], 5));
    }
}
