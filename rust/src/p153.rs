/*
Problem 153. Find Minimum in Rotated Sorted Array
=================================================

https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/

Suppose an array of length n sorted in ascending order is rotated between 1 and n times. For
example, the array nums = [0,1,2,4,5,6,7] might become:

    - [4,5,6,7,0,1,2] if it was rotated 4 times.
    - [0,1,2,4,5,6,7] if it was rotated 7 times.

Notice that rotating an array [a[0], a[1], a[2], ..., a[n-1]] 1 time results in the array [a[n-1],
a[0], a[1], a[2], ..., a[n-2]].

Given the sorted rotated array nums, return the minimum element of this array.

Example 1:

    Input: nums = [3,4,5,1,2]
    Output: 1
    Explanation: The original array was [1,2,3,4,5] rotated 3 times.

Example 2:

    Input: nums = [4,5,6,7,0,1,2]
    Output: 0
    Explanation: The original array was [0,1,2,4,5,6,7] and it was rotated 4 times.

Example 3:

    Input: nums = [11,13,15,17]
    Output: 11
    Explanation: The original array was [11,13,15,17] and it was rotated 4 times. 

Constraints:

    - n == nums.length
    - 1 <= n <= 5000
    - -5000 <= nums[i] <= 5000
    - All the integers of nums are unique.
    - nums is sorted and rotated between 1 and n times.
*/

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut lb, mut rb) = (0, nums.len());
        while rb - lb > 10 {
            let lmn = nums[lb];
            let rmn = nums[rb - 1];
            if lmn < rmn {
                return lmn;
            }
            let mid = (lb + rb) / 2;
            let midn = nums[mid];
            if midn < nums[mid - 1] {
                return nums[mid];
            }
            if midn > lmn {
                lb = mid + 1;
            } else {
                rb = mid;
            }
        }
        nums[lb..rb].iter().min().cloned().unwrap_or(-1)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!( Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
    }

    #[test]
    fn test_case2() {
        assert_eq!( Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    }

    #[test]
    fn test_case3() {
        assert_eq!( Solution::find_min(vec![11, 13, 15, 17]), 11);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]));
    }
}
