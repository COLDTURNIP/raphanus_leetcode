/*
Problem 922. Sort Array By Parity II
====================================

https://leetcode.com/problems/sort-array-by-parity-ii/

Given an array A of non-negative integers, half of the integers in A are odd, and half of the
integers are even.

Sort the array so that whenever A[i] is odd, i is odd; and whenever A[i] is even, i is even.

You may return any answer array that satisfies this condition.

Example 1:

    Input: [4,2,5,7]
    Output: [4,5,2,7]
    Explanation: [4,7,2,5], [2,5,4,7], [2,7,4,5] would also have been accepted.

Note:

    - 2 <= A.length <= 20000
    - A.length % 2 == 0
    - 0 <= A[i] <= 1000
*/

impl Solution {
    pub fn sort_array_by_parity_ii(mut a: Vec<i32>) -> Vec<i32> {
        let len = a.len();
        let mut even_iter = (0..len).step_by(2);
        let mut odd_iter = (1..len).step_by(2);
        while let Some(i) = even_iter.find(|&i| a[i] & 1 == 1) {
            let j = odd_iter.find(|&j| a[j] & 1 == 0).unwrap();
            a.swap(i, j);
        }
        a
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    fn check(input: Vec<i32>) {
        assert!(Solution::sort_array_by_parity_ii(input)
            .into_iter()
            .enumerate()
            .all(|(i, n)| i as i32 & 1 == n & 1));
    }

    #[test]
    fn test_4257() {
        check(vec![4, 2, 5, 7]);
    }

    #[test]
    fn test_empty() {
        check(Vec::new());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::sort_array_by_parity_ii(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    }
}
