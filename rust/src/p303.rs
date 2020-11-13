/*
Problem 303. Range Sum Query - Immutable
========================================

https://leetcode.com/problems/range-sum-query-immutable/

Given an integer array nums, find the sum of the elements between indices i and j (i ≤ j),
inclusive.

Implement the NumArray class:

    - NumArray(int[] nums) Initializes the object with the integer array nums.

    - int sumRange(int i, int j) Return the sum of the elements of the nums array in the range
      [i, j] inclusive (i.e., sum(nums[i], nums[i + 1], ... , nums[j]))

Example 1:

    Input
    ["NumArray", "sumRange", "sumRange", "sumRange"]
    [[[-2, 0, 3, -5, 2, -1]], [0, 2], [2, 5], [0, 5]]
    Output
    [null, 1, -1, -3]

    Explanation
    NumArray numArray = new NumArray([-2, 0, 3, -5, 2, -1]);
    numArray.sumRange(0, 2); // return 1 ((-2) + 0 + 3)
    numArray.sumRange(2, 5); // return -1 (3 + (-5) + 2 + (-1))
    numArray.sumRange(0, 5); // return -3 ((-2) + 0 + 3 + (-5) + 2 + (-1))

Constraints:

    - 0 <= nums.length <= 10^4
    - -10.5 <= nums[i] <= 10^5
    - 0 <= i <= j < nums.length
    - At most 10^4 calls will be made to sumRange.
*/

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(i, j);
 */

struct NumArray {
    acc_data: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        NumArray {
            acc_data: nums
                .into_iter()
                .scan(0, |acc, n| {
                    *acc += n;
                    Some(*acc)
                })
                .collect(),
        }
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        let i = i as usize;
        let j = j as usize;
        self.acc_data[j] - i.checked_sub(1).map(|i| self.acc_data[i]).unwrap_or(0)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::NumArray;

    #[test]
    fn test_numarray() {
        let na = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(na.sum_range(0, 2), 1);
        assert_eq!(na.sum_range(2, 5), -1);
        assert_eq!(na.sum_range(0, 5), -3);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| NumArray::new(vec![-2, 0, 3, -5, 2, -1]).sum_range(0, 2));
    }
}
