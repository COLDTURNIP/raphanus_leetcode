/*
Problem 27. Remove Element
=======================================

https://leetcode.com/problems/remove-element/

Given an array nums and a value val, remove all instances of that value in-place and return the new
length.

Do not allocate extra space for another array, you must do this by modifying the input array
in-place with O(1) extra memory.

The order of elements can be changed. It doesn't matter what you leave beyond the new length.
*/
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut cur: usize = 0;
        for idx in 0..nums.len() {
            let cur_val = nums[idx];
            if cur_val != val {
                nums[cur] = cur_val;
                cur += 1;
            }
        }
        cur as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_reverse_k_group() {
        assert_eq!(Solution::remove_element(&mut vec![3, 2, 2, 3], 2), 2);
        assert_eq!(
            Solution::remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2),
            5
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2));
    }
}
