/*
Problem 659. Split Array into Consecutive Subsequences
======================================================

https://leetcode.com/problems/split-array-into-consecutive-subsequences/

Given an array nums sorted in ascending order, return true if and only if you can split it into 1
or more subsequences such that each subsequence consists of consecutive integers and has length at
least 3.

Example 1:

    Input: [1,2,3,3,4,5]
    Output: True
    Explanation:
    You can split them into two consecutive subsequences :
    1, 2, 3
    3, 4, 5

Example 2:

    Input: [1,2,3,3,4,4,5,5]
    Output: True
    Explanation:
    You can split them into two consecutive subsequences :
    1, 2, 3, 4, 5
    3, 4, 5

Example 3:

    Input: [1,2,3,4,4,5]
    Output: False

Constraints:

    1 <= nums.length <= 10000
*/

impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut num_cnt = {
            let mut cnt = Vec::<(i32, usize, usize)>::with_capacity(nums.len());
            for n in nums {
                match cnt.last_mut() {
                    Some(nc) if nc.0 == n => nc.1 += 1,
                    _ => cnt.push((n, 1, 0)),
                }
            }
            cnt
        };
        let mut last_cnt = 0;
        for i in 0..num_cnt.len() {
            let (cur_n, cur_cnt, _) = num_cnt[i];
            let need = cur_cnt.saturating_sub(last_cnt);
            if need > 0 {
                for &step in &[1, 2] {
                    if let Some(&mut (next_n, next_cnt, ref mut next_need)) =
                        num_cnt.get_mut(i + step)
                    {
                        if next_n != cur_n + step as i32 {
                            return false;
                        }
                        *next_need += need;
                        if *next_need > next_cnt {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
            last_cnt = cur_cnt;
        }
        true
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert!(Solution::is_possible(vec![1, 2, 3, 3, 4, 5]));
    }

    #[test]
    fn test_case2() {
        assert!(Solution::is_possible(vec![1, 2, 3, 3, 4, 4, 5, 5]));
    }

    #[test]
    fn test_case3() {
        assert!(!Solution::is_possible(vec![1, 2, 3, 4, 4, 5]));
    }

    #[test]
    fn test_empty() {
        assert!(Solution::is_possible(Vec::new()));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::is_possible(vec![1, 2, 3, 3, 4, 4, 5, 5]);
        });
    }
}
