/*
Problem 89. Gray Code
=====================

https://leetcode.com/problems/gray-code/

The gray code is a binary numeral system where two successive values differ in only one bit.

Given a non-negative integer n representing the total number of bits in the code, print the
sequence of gray code. A gray code sequence must begin with 0.

Example 1:

    Input: 2
    Output: [0,1,3,2]
    Explanation:
    00 - 0
    01 - 1
    11 - 3
    10 - 2

    For a given n, a gray code sequence may not be uniquely defined.
    For example, [0,2,3,1] is also a valid gray code sequence.

    00 - 0
    10 - 2
    11 - 3
    01 - 1

Example 2:

    Input: 0
    Output: [0]
    Explanation: We define the gray code sequence to begin with 0.
                 A gray code sequence of n has size = 2n, which for n = 0 the size is 20 = 1.
                 Therefore, for n = 0 the gray code sequence is [0].
*/

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        // refer to communication theory, encoding
        match n {
            n if n < 0 => Self::gray_code(0),
            0 => vec![0],
            n => {
                let mut ans = Self::gray_code(n - 1);
                let heading = 1 << (n - 1);
                for i in (0..ans.len()).rev() {
                    ans.push(heading | ans[i]);
                }
                ans
            }
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    fn check(bits: i32, expect_len: usize) {
        let output = Solution::gray_code(bits);
        assert_eq!(output.len(), expect_len);
        for p in output.windows(2) {
            assert_eq!((p[0] ^ p[1]).count_ones(), 1);
        }
    }

    #[test]
    fn test_2() {
        check(2, 4);
    }

    #[test]
    fn test_0() {
        check(0, 1);
        assert_eq!(Solution::gray_code(0), vec![0]);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::gray_code(10));
    }
}
