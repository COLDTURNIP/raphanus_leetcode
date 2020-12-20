/*
Problem 474. Ones and Zeroes
============================

https://leetcode.com/problems/ones-and-zeroes/

You are given an array of binary strings strs and two integers m and n.

Return the size of the largest subset of strs such that there are at most m 0's and n 1's in the
subset.

A set x is a subset of a set y if all elements of x are also elements of y.

Example 1:

    Input: strs = ["10","0001","111001","1","0"], m = 5, n = 3
    Output: 4
    Explanation: The largest subset with at most 5 0's and 3 1's is {"10", "0001", "1", "0"}, so
    the answer is 4.
    Other valid but smaller subsets include {"0001", "1"} and {"10", "1", "0"}.
    {"111001"} is an invalid subset because it contains 4 1's, greater than the maximum of 3.

Example 2:

    Input: strs = ["10","0","1"], m = 1, n = 1
    Output: 2
    Explanation: The largest subset is {"0", "1"}, so the answer is 2.

Constraints:

    - 1 <= strs.length <= 600
    - 1 <= strs[i].length <= 100
    - strs[i] consists only of digits '0' and '1'.
    - 1 <= m, n <= 100
*/

impl Solution {
    #[allow(clippy::naive_bytecount)]
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut count = vec![vec![0; n + 1]; m + 1];
        for cur_str in strs {
            let cnt = {
                let len = cur_str.len();
                let cnt0 = cur_str.as_bytes().iter().filter(|&&c| b'0' == c).count();
                (cnt0, len - cnt0)
            };
            for rest_m in (cnt.0..=m).rev() {
                for rest_n in (cnt.1..=n).rev() {
                    count[rest_m][rest_n] =
                        count[rest_m][rest_n].max(1 + count[rest_m - cnt.0][rest_n - cnt.1]);
                }
            }
        }
        count[m][n]
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
            Solution::find_max_form(
                vec![
                    "10".to_owned(),
                    "0001".to_owned(),
                    "111001".to_owned(),
                    "1".to_owned(),
                    "0".to_owned()
                ],
                5,
                3
            ),
            4
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::find_max_form(vec!["10".to_owned(), "0".to_owned(), "1".to_owned()], 1, 1),
            2
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::find_max_form(Vec::new(), 999, 98), 0);
        assert_eq!(
            Solution::find_max_form(vec!["1100".to_owned(), "0101".to_owned()], 1, 1),
            0
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::find_max_form(
                vec![
                    "10".to_owned(),
                    "0001".to_owned(),
                    "111001".to_owned(),
                    "1".to_owned(),
                    "0".to_owned(),
                ],
                5,
                3,
            )
        });
    }
}
