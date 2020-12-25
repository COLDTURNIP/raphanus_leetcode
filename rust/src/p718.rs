/*
Problem 718. Maximum Length of Repeated Subarray
================================================

https://leetcode.com/problems/maximum-length-of-repeated-subarray/

Given two integer arrays A and B, return the maximum length of an subarray that appears in both
arrays.

Example 1:

    Input:
    A: [1,2,3,2,1]
    B: [3,2,1,4,7]
    Output: 3
    Explanation:
    The repeated subarray with maximum length is [3, 2, 1].

Note:

    - 1 <= len(A), len(B) <= 1000
    - 0 <= A[i], B[i] < 100
*/

impl Solution {
    pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let (a, b) = if a.len() > b.len() { (a, b) } else { (b, a) };
        let mut match_state = vec![0; b.len()];
        let mut max_len = 0;
        for n in a {
            for bi in (0..b.len()).rev() {
                if b[bi] == n {
                    match_state[bi] = bi
                        .checked_sub(1)
                        .map(|i| 1 + match_state[i])
                        .unwrap_or(1);
                    max_len = max_len.max(match_state[bi]);
                } else {
                    match_state[bi] = 0;
                }
            }
        }
        max_len
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7],),
            3
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7]));
    }
}
