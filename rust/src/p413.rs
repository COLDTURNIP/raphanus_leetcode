/*
Problem 413. Arithmetic Slices
==============================

https://leetcode.com/problems/arithmetic-slices/

A sequence of numbers is called arithmetic if it consists of at least three elements and if the
difference between any two consecutive elements is the same.

For example, these are arithmetic sequences:

    1, 3, 5, 7, 9
    7, 7, 7, 7
    3, -1, -5, -9

The following sequence is not arithmetic.

    1, 1, 2, 5, 7

A zero-indexed array A consisting of N numbers is given. A slice of that array is any pair of
integers (P, Q) such that 0 <= P < Q < N.

A slice (P, Q) of the array A is called arithmetic if the sequence:
A[P], A[P + 1], ..., A[Q - 1], A[Q] is arithmetic. In particular, this means that P + 1 < Q.

The function should return the number of arithmetic slices in the array A.

Example:

    A = [1, 2, 3, 4]

    return: 3, for 3 arithmetic slices in A: [1, 2, 3], [2, 3, 4] and [1, 2, 3, 4] itself.
*/

impl Solution {
    pub fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
        if a.len() < 3 {
            return 0;
        }
        let mut ans = 0;
        let mut diff = a[1] - a[0];
        let mut seq_len = 2;
        let mut last = a[1];
        for n in a.into_iter().skip(2) {
            let cur_diff = n - last;
            if cur_diff == diff {
                seq_len += 1;
            } else {
                ans += (seq_len - 2) * (seq_len - 1) / 2;
                diff = cur_diff;
                seq_len = 2;
            }
            last = n;
        }
        ans + (seq_len - 2) * (seq_len - 1) / 2
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_arithmetic_count() {
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4]), 3);
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4, 2, 0, -2, -4]),
            9
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4, 5, 6, 7, 8]));
    }
}
