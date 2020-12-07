/*
Problem 740. Delete and Earn
============================

https://leetcode.com/problems/delete-and-earn/

Given an array nums of integers, you can perform operations on the array.

In each operation, you pick any nums[i] and delete it to earn nums[i] points. After, you must
delete every element equal to nums[i] - 1 or nums[i] + 1.

You start with 0 points. Return the maximum number of points you can earn by applying such
operations.

Example 1:

    Input: nums = [3, 4, 2]
    Output: 6
    Explanation:
    Delete 4 to earn 4 points, consequently 3 is also deleted.
    Then, delete 2 to earn 2 points. 6 total points are earned.

Example 2:

    Input: nums = [2, 2, 3, 3, 3, 4]
    Output: 9
    Explanation:
    Delete 3 to earn 3 points, deleting both 2's and the 4.
    Then, delete 3 again to earn 3 points, and 3 again to earn 3 points.
    9 total points are earned.

Note:

    - The length of nums is at most 20000.
    - Each element nums[i] is an integer in the range [1, 10000].
*/

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let num_cnts = {
            let mut nums = nums;
            nums.sort_unstable();
            let mut cnts = Vec::<(i32, i32)>::with_capacity(nums.len());
            let mut last = -1;
            for n in nums.iter() {
                if *n != last {
                    cnts.push((*n, 1));
                    last = *n;
                } else if let Some(p) = cnts.last_mut() {
                    p.1 += 1
                }
            }
            cnts
        };
        let (mut sc, mut sc_last, mut sc_last2) = (0, 0, 0);
        for i in 0..num_cnts.len() {
            sc_last2 = sc_last2.max(sc_last);
            sc_last = sc;
            let (n, cnt) = num_cnts[i];
            if n - 1 == i.checked_sub(1).map(|i| num_cnts[i].0).unwrap_or(-1) {
                sc = n * cnt + sc_last2;
            } else {
                sc = n * cnt + sc_last.max(sc_last2);
            }
        }
        sc.max(sc_last)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(Solution::delete_and_earn(vec![3, 4, 2]), 6);
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::delete_and_earn(vec![2, 2, 3, 4, 3, 3]), 9);
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::delete_and_earn(vec![1, 1, 1, 2, 4, 5, 5, 5, 6]),
            18
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::delete_and_earn(Vec::new()), 0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::delete_and_earn(vec![2, 2, 3, 4, 3, 3]));
    }
}
