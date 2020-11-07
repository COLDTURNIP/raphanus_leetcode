/*
Problem 628. Maximum Product of Three Numbers
=============================================

https://leetcode.com/problems/maximum-product-of-three-numbers/

Given an integer array nums, find three numbers whose product is maximum and return the maximum
product.

Example 1:

    Input: nums = [1,2,3]
    Output: 6

Example 2:

    Input: nums = [1,2,3,4]
    Output: 24

Example 3:

    Input: nums = [-1,-2,-3]
    Output: -6

Constraints:

    - 3 <= nums.length <= 104
    - -1000 <= nums[i] <= 1000
*/

impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut mins = nums.iter().take(3).cloned().collect::<Vec<i32>>();
        let mut maxs = nums.iter().take(3).cloned().collect::<Vec<i32>>();
        mins.sort_unstable();
        maxs.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
        for n in nums.into_iter().skip(3) {
            match mins.binary_search(&n) {
                Ok(i) | Err(i) if i < 2 => {
                    mins.insert(i, n);
                    mins.sort_unstable();
                    mins.truncate(2);
                }
                _ => (),
            };
            match maxs.binary_search_by(|item| n.cmp(item)) {
                Ok(i) | Err(i) if i < 3 => {
                    maxs.insert(i, n);
                    mins.sort_unstable();
                    maxs.truncate(3);
                }
                _ => (),
            }
        }
        (maxs[0] * maxs[1] * maxs[2]).max(maxs[0] * mins[0] * mins[1])
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_123() {
        assert_eq!(Solution::maximum_product(vec![1, 2, 3]), 6)
    }

    #[test]
    fn test_1234() {
        assert_eq!(Solution::maximum_product(vec![1, 2, 3, 4]), 24)
    }

    #[test]
    fn test_neg_123() {
        assert_eq!(Solution::maximum_product(vec![-1, -2, -3]), -6)
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::maximum_product(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]));
    }
}
