/*
Problem 26. Remove Duplicates from Sorted Array
=======================================

https://leetcode.com/problems/remove-duplicates-from-sorted-array/

*/

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut ret_len: usize = 0;
        let mut last_val = None;
        for idx in 0..nums.len() {
            let chk_val = nums[idx];
            if match last_val {
                Some(last_val) => last_val != chk_val,
                None => true,
            } {
                nums[ret_len] = chk_val;
                last_val = Some(chk_val);
                ret_len += 1;
            }
        }
        ret_len as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let mut v = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut v), 2);
        assert_eq!(&v[0..2], [1, 2]);

        let mut v = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates(&mut v), 5);
        assert_eq!(&v[0..5], [0, 1, 2, 3, 4]);
    }
}
