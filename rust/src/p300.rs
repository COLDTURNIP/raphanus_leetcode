/*
300. Longest Increasing Subsequence
=======================================

https://leetcode.com/problems/longest-increasing-subsequence/

Given an unsorted array of integers, find the length of longest increasing subsequence.

Example:

    Input: [10,9,2,5,3,7,101,18]
    Output: 4
    Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
    Note:

There may be more than one LIS combination, it is only necessary for you to return the length.
Your algorithm should run in O(n2) complexity.
Follow up: Could you improve it to O(n log n) time complexity?

*/

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut possible_seq = Vec::with_capacity(nums.len());
        for n in nums.iter() {
            match possible_seq.binary_search(n) {
                Err(pos) if pos >= possible_seq.len() => possible_seq.push(*n),
                Err(pos) => possible_seq[pos] = *n,
                _ => (),
            }
        }
        possible_seq.len() as i32
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
        assert_eq!(Solution::length_of_lis(vec![1, 2]), 2);
        assert_eq!(Solution::length_of_lis(vec![7, 1, 5, 3, 6, 4]), 3);
        assert_eq!(Solution::length_of_lis(vec![1, 2, 3, 4, 5]), 5);
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use super::Solution;

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]));
    }
}
