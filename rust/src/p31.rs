/*
Problem 31. Next Permutation
============================

https://leetcode.com/problems/next-permutation/

Implement next permutation, which rearranges numbers into the lexicographically next greater
permutation of numbers.

If such arrangement is not possible, it must rearrange it as the lowest possible order (ie, sorted
in ascending order).

The replacement must be in-place and use only constant extra memory.

Here are some examples. Inputs are in the left-hand column and its corresponding outputs are in the
right-hand column.

1,2,3 → 1,3,2
3,2,1 → 1,2,3
1,1,5 → 1,5,1
*/

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let len = nums.len();
        if len < 2 {
            return;
        }

        // step 1. Find the first decrease point as swap_idx from right to left
        let swap_idx = (1..=len - 1)
            .rev()
            .find(|&i| nums[i] > nums[i - 1])
            .unwrap_or(0);

        // step 2. Reverse nums[swap_idx..len], and find the lease number greater to swap_idx-1
        let old_left_i = swap_idx.saturating_sub(1);
        let old_left = nums[old_left_i];
        let mut new_left_i = len;
        let (mut li, mut ri) = (swap_idx, len - 1);
        while li <= ri {
            nums.swap(li, ri);
            if nums[li] > old_left && new_left_i > li {
                new_left_i = li;
            }
            if nums[ri] > old_left && new_left_i > ri {
                new_left_i = ri;
            }
            li += 1;
            ri -= 1;
        }
        if new_left_i < len {
            nums.swap(old_left_i, new_left_i);
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    fn check_case(mut nums: Vec<i32>, expect: Vec<i32>) {
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, expect);
    }

    #[test]
    fn test_123() {
        check_case(vec![1, 2, 3], vec![1, 3, 2]);
    }

    #[test]
    fn test_321() {
        check_case(vec![3, 2, 1], vec![1, 2, 3]);
    }

    #[test]
    fn test_132() {
        check_case(vec![1, 3, 2], vec![2, 1, 3]);
    }

    #[test]
    fn test_115() {
        check_case(vec![1, 1, 5], vec![1, 5, 1]);
    }

    #[test]
    fn test_3412() {
        check_case(vec![3, 5, 4, 2, 1], vec![4, 1, 2, 3, 5]);
    }

    #[test]
    fn test_len1() {
        check_case(vec![99], vec![99]);
    }

    #[test]
    fn test_empty() {
        check_case(Vec::new(), Vec::new());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::next_permutation(&mut vec![1, 3, 4, 6, 8, 7, 2, 9]));
    }
}
