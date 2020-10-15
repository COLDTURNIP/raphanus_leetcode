/*
Problem 969. Pancake Sorting
============================

https://leetcode.com/problems/pancake-sorting/

Given an array of integers arr, sort the array by performing a series of pancake flips.

In one pancake flip we do the following steps:

    Choose an integer k where 1 <= k <= arr.length.
    Reverse the sub-array arr[1...k].

For example, if arr = [3,2,1,4] and we performed a pancake flip choosing k = 3, we reverse the
sub-array [3,2,1], so arr = [1,2,3,4] after the pancake flip at k = 3.

Return the k-values corresponding to a sequence of pancake flips that sort arr. Any valid answer
that sorts the array within 10 * arr.length flips will be judged as correct.

Example 1:

    Input: arr = [3,2,4,1]
    Output: [4,2,4,3]
    Explanation:
    We perform 4 pancake flips, with k values 4, 2, 4, and 3.
    Starting state: arr = [3, 2, 4, 1]
    After 1st flip (k = 4): arr = [1, 4, 2, 3]
    After 2nd flip (k = 2): arr = [4, 1, 2, 3]
    After 3rd flip (k = 4): arr = [3, 2, 1, 4]
    After 4th flip (k = 3): arr = [1, 2, 3, 4], which is sorted.
    Notice that we return an array of the chosen k values of the pancake flips.

Example 2:

    Input: arr = [1,2,3]
    Output: []
    Explanation: The input is already sorted, so there is no need to flip anything.
    Note that other answers, such as [3, 3], would also be accepted.

Constraints:

    - 1 <= arr.length <= 100
    - 1 <= arr[i] <= arr.length
    - All integers in arr are unique (i.e. arr is a permutation of the integers from 1 to
      arr.length).
*/

impl Solution {
    pub fn pancake_sort(mut arr: Vec<i32>) -> Vec<i32> {
        // core idea: given subslice, find the maximum entry in the array, and flip it to the
        // most right place.
        let mut steps = Vec::new();
        for len in (2..=arr.len()).rev() {
            if let Some(max_idx) = (0..len).max_by_key(|&i| arr[i]) {
                if max_idx == len - 1 {
                    continue;
                }
                steps.push(max_idx as i32 + 1);
                for i in 0..(max_idx + 1) / 2 {
                    arr.swap(i, max_idx - i);
                }
                steps.push(len as i32);
                for i in 0..len / 2 {
                    arr.swap(i, len - 1 - i);
                }
            }
        }
        steps
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    fn check(mut input: Vec<i32>) {
        let mut sorted = input.clone();
        sorted.sort_unstable();
        for flip in Solution::pancake_sort(input.clone())
            .into_iter()
            .map(|i| i as usize - 1)
        {
            for i in 0..(flip + 1) / 2 {
                input.swap(i, flip - i);
            }
        }
        assert_eq!(input, sorted);
    }

    #[test]
    fn test_3241() {
        check(vec![3, 2, 4, 1]);
    }

    #[test]
    fn test_123() {
        check(vec![1, 2, 3]);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::pancake_sort(Vec::new()), Vec::<i32>::new());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::pancake_sort(vec![0, 2, 3, 4, 5, 7, 9, 8]));
    }
}
