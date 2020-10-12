/*
Problem 414. Third Maximum Number
=================================

https://leetcode.com/problems/third-maximum-number/

Given a non-empty array of integers, return the third maximum number in this array. If it does not
exist, return the maximum number. The time complexity must be in O(n).

Example 1:

    Input: [3, 2, 1]

    Output: 1

    Explanation: The third maximum is 1.

Example 2:

    Input: [1, 2]

    Output: 2

    Explanation: The third maximum does not exist, so the maximum (2) is returned instead.

Example 3:

    Input: [2, 2, 3, 1]

    Output: 1

    Explanation: Note that the third maximum here means the third maximum distinct number.
    Both numbers with value 2 are both considered as second maximum.
*/

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let max_len = 3;
        let mut buf = Vec::new();
        for n in nums.into_iter() {
            match buf.binary_search_by(|seen| n.cmp(seen)) {
                Err(i) if i < max_len => {
                    buf.insert(i, n);
                    buf.truncate(max_len);
                }
                _ => {}
            }
        }
        if buf.len() < max_len {
            *buf.first().unwrap()
        } else {
            *buf.last().unwrap()
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_321() {
        assert_eq!(Solution::third_max(vec![3, 2, 1]), 1);
    }

    #[test]
    fn test_12() {
        assert_eq!(Solution::third_max(vec![1, 2]), 2);
    }

    #[test]
    fn test_2231() {
        assert_eq!(Solution::third_max(vec![2, 2, 3, 1]), 1);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::third_max(vec![2, 7, 11, 15]));
    }
}
