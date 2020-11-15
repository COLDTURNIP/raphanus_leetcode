/*
Problem 338. Counting Bits
==========================

https://leetcode.com/problems/counting-bits/

Given a non negative integer number num. For every numbers i in the range 0 ≤ i ≤ num calculate the
number of 1's in their binary representation and return them as an array.

Example 1:

    Input: 2
    Output: [0,1,1]

Example 2:

    Input: 5
    Output: [0,1,1,2,1,2]

Follow up:

    - It is very easy to come up with a solution with run time O(n*sizeof(integer)). But can you do
      it in linear time O(n) /possibly in a single pass?
    - Space complexity should be O(n).
    - Can you do it like a boss? Do it without using any builtin function like __builtin_popcount
      in c++ or in any other language.
*/

impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut ans = Vec::with_capacity(num as usize);
        ans.push(0);
        for n in 1..=num {
            ans.push(ans[(n & (n - 1)) as usize] + 1);
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
    fn test_2() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }

    #[test]
    fn test_8() {
        assert_eq!(Solution::count_bits(8), vec![0, 1, 1, 2, 1, 2, 2, 3, 1]);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::count_bits(33));
    }
}
