/*
Problem 1027. Longest Arithmetic Subsequence
============================================

https://leetcode.com/problems/longest-arithmetic-subsequence/

Given an array A of integers, return the length of the longest arithmetic subsequence in A.

Recall that a subsequence of A is a list A[i_1], A[i_2], ..., A[i_k] with 0 <= i_1 < i_2 < ... <
i_k <= A.length - 1, and that a sequence B is arithmetic if B[i+1] - B[i] are all the same value
(for 0 <= i < B.length - 1).

Example 1:

    Input: A = [3,6,9,12]
    Output: 4
    Explanation:
    The whole array is an arithmetic sequence with steps of length = 3.

Example 2:

    Input: A = [9,4,7,2,10]
    Output: 3
    Explanation:
    The longest arithmetic subsequence is [4,7,10].

Example 3:

    Input: A = [20,1,15,3,10,5,8]
    Output: 4
    Explanation:
    The longest arithmetic subsequence is [20,15,10,5].

Constraints:

    - 2 <= A.length <= 1000
    - 0 <= A[i] <= 500
*/

use std::collections::HashMap;

impl Solution {
    pub fn longest_arith_seq_length(a: Vec<i32>) -> i32 {
        // max_len[i][d] means: for subseq ends at a[i], the longest length of arithmetic seq with
        // step d
        // NOTE: here we use HashMap as sparse array. The faster way is to use a Vec of length
        // 500*2+1.
        let mut max_len: Vec<HashMap<i32, i32>> = Vec::with_capacity(a.len());

        let mut ans = 2;
        for (i, n) in a.iter().enumerate() {
            let mut cur_max_len = HashMap::new();
            for j in (0..i).rev() {
                let diff = n - a[j];
                if cur_max_len.contains_key(&diff) {
                    continue;
                }
                let d_max_len = max_len[j].get(&diff).map(|&len| len + 1).unwrap_or(2);
                cur_max_len.insert(diff, d_max_len);
                ans = ans.max(d_max_len);
            }
            max_len.push(cur_max_len);
        }
        ans
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(Solution::longest_arith_seq_length(vec![3, 6, 9, 12]), 4);
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::longest_arith_seq_length(vec![9, 4, 7, 2, 10]), 3);
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::longest_arith_seq_length(vec![20, 1, 15, 3, 10, 5, 8]),
            4
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::longest_arith_seq_length(vec![20, 1, 15, 3, 10, 5, 8]));
    }
}
