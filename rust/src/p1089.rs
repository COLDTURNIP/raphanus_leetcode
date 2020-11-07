/*
Problem 1089. Duplicate Zeros
=============================

https://leetcode.com/problems/duplicate-zeros/

Given a fixed length array arr of integers, duplicate each occurrence of zero, shifting the
remaining elements to the right.

Note that elements beyond the length of the original array are not written.

Do the above modifications to the input array in place, do not return anything from your function.

Example 1:

    Input: [1,0,2,3,0,4,5,0]
    Output: null
    Explanation: After calling your function, the input array is modified to: [1,0,0,2,3,0,0,4]

Example 2:

    Input: [1,2,3]
    Output: null
    Explanation: After calling your function, the input array is modified to: [1,2,3]

Note:

    - 1 <= arr.length <= 10000
    - 0 <= arr[i] <= 9
*/

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let len = arr.len();
        let mut i = 0;
        let mut shift = 0;
        while i + shift < len {
            shift += (arr[i] == 0) as usize;
            i += 1;
        }
        for i in (0..i).rev() {
            if arr[i] == 0 {
                if let Some(entry) = arr.get_mut(i + shift) {
                    *entry = 0;
                }
                shift -= 1;
                arr[i + shift] = 0;
            } else {
                arr[i + shift] = arr[i];
            }
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    fn check(mut input: Vec<i32>, expect: Vec<i32>) {
        Solution::duplicate_zeros(&mut input);
        assert_eq!(input, expect);
    }

    #[test]
    fn test_two_zeros() {
        check(vec![1, 0, 2, 3, 0, 4, 5, 0], vec![1, 0, 0, 2, 3, 0, 0, 4]);
        check(
            vec![1, 5, 2, 0, 6, 8, 0, 6, 0],
            vec![1, 5, 2, 0, 0, 6, 8, 0, 0],
        );
    }

    #[test]
    fn test_no_zero() {
        check(vec![1, 2, 3], vec![1, 2, 3]);
    }

    #[test]
    fn test_empty() {
        check(Vec::new(), Vec::new());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::duplicate_zeros(&mut vec![0; 100]));
    }
}
