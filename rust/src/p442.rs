/*
Problem 442. Find All Duplicates in an Array
============================================

https://leetcode.com/problems/find-all-duplicates-in-an-array/

Given an array of integers, 1 ≤ a[i] ≤ n (n = size of array), some elements appear twice and others
appear once.

Find all the elements that appear twice in this array.

Could you do it without extra space and in O(n) runtime?

Example:
    Input:
    [4,3,2,7,8,2,3,1]

    Output:
    [2,3]
*/

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        // two point to let us treat the nums array itself as a hash table:
        //     1. 1 ≤ a[i] ≤ n
        //     2. duplicated elements appear exactly twice, no more, no less
        let mut dup = Vec::with_capacity(nums.len() / 2);
        for i in 0..nums.len() {
            let n = nums[i].abs();
            let k = n as usize - 1;
            if nums[k] < 0 {
                dup.push(n);
            } else {
                nums[k] = -nums[k];
            }
        }
        dup
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    fn check(input: Vec<i32>, mut expect: Vec<i32>) {
        let mut exact = Solution::find_duplicates(input);
        exact.sort_unstable();
        expect.sort_unstable();
        assert_eq!(exact, expect);
    }

    #[test]
    fn test_duplicated() {
        check(vec![4, 3, 2, 7, 8, 2, 3, 1], vec![2, 3]);
    }

    #[test]
    fn test_empty() {
        check(Vec::new(), Vec::new());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]));
    }
}
