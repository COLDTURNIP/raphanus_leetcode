/*
Problem 80. Remove Duplicates from Sorted Array II
==================================================

https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/

Given a sorted array nums, remove the duplicates in-place such that duplicates appeared at most
twice and return the new length.

Do not allocate extra space for another array, you must do this by modifying the input array
in-place with O(1) extra memory.

Example 1:

    Given nums = [1,1,1,2,2,3],

    Your function should return length = 5, with the first five elements of nums being 1, 1, 2, 2
    and 3 respectively.

    It doesn't matter what you leave beyond the returned length.

Example 2:

    Given nums = [0,0,1,1,1,1,2,3,3],

    Your function should return length = 7, with the first seven elements of nums being modified to
    0, 0, 1, 1, 2, 3 and 3 respectively.

    It doesn't matter what values are set beyond the returned length.

Clarification:

    Confused why the returned value is an integer but your answer is an array?

    Note that the input array is passed in by reference, which means modification to the input array will be known to the caller as well.

    Internally you can think of this:

        // nums is passed in by reference. (i.e., without making a copy)
        int len = removeDuplicates(nums);

        // any modification to nums in your function would be known by the caller.
        // using the length returned by your function, it prints the first len elements.
        for (int i = 0; i < len; i++) {
            print(nums[i]);
        }
*/

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        const LIMIT: usize = 2;

        let mut cur_mod = 0;
        let mut last = None;
        let mut cnt = 0;
        for cur_idx in 0..nums.len() {
            let cur = Some(nums[cur_idx]);
            if cur != last {
                last = cur;
                cnt = 1;
            } else {
                cnt += 1;
            }
            if cnt <= LIMIT {
                nums[cur_mod] = nums[cur_idx];
                cur_mod += 1;
            }
        }
        nums.truncate(cur_mod);
        cur_mod as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    fn check(mut nums: Vec<i32>, expect: Vec<i32>) {
        let len = Solution::remove_duplicates(&mut nums);
        assert_eq!(nums.len(), len as usize);
        assert_eq!(nums, expect.as_slice());
    }

    #[test]
    fn test_111223() {
        check(vec![1, 1, 1, 2, 2, 3], vec![1, 1, 2, 2, 3])
    }

    #[test]
    fn test_001111233() {
        check(vec![0, 0, 1, 1, 1, 1, 2, 3, 3], vec![0, 0, 1, 1, 2, 3, 3])
    }

    #[test]
    fn test_zero() {
        check(Vec::new(), Vec::new());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 1, 2, 3, 3]));
    }
}
