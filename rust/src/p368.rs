/*
Problem 368. Largest Divisible Subset
==========================================================

https://leetcode.com/problems/largest-divisible-subset/

Given a set of distinct positive integers, find the largest subset such that every pair (Si, Sj) of
elements in this subset satisfies:

Si % Sj = 0 or Sj % Si = 0.

If there are multiple solutions, return any subset is fine.

Example 1:

    Input: [1,2,3]
    Output: [1,2] (of course, [1,3] will also be ok)

Example 2:

    Input: [1,2,4,8]
    Output: [1,2,4,8]
*/

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return Vec::new();
        }
        nums.sort_unstable();
        let mut cnt = Vec::new();
        let mut tail_len: usize = 0;
        let mut tail_idx: usize = 0;
        for (i, &n) in nums.iter().enumerate() {
            let (max_len, prev) = cnt
                .iter()
                .enumerate()
                .take(i)
                .filter_map(|(j, &(len, _prev))| {
                    if n % nums[j] == 0 {
                        Some((len + 1, j))
                    } else {
                        None
                    }
                })
                .max()
                .unwrap_or((1, i));
            cnt.push((max_len, prev));
            if tail_len < max_len {
                tail_len = max_len;
                tail_idx = i;
            }
        }
        let mut ans = Vec::with_capacity(tail_len);
        for _len in 0..tail_len {
            ans.push(nums[tail_idx]);
            tail_idx = cnt[tail_idx].1;
        }
        ans
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;
    use std::collections::HashSet;

    fn check(nums: Vec<i32>, expect_size: usize) {
        let exact_vec = Solution::largest_divisible_subset(nums);
        assert_eq!(exact_vec.len(), expect_size);
        if !exact_vec.is_empty() {
            for i in 0..exact_vec.len() - 1 {
                for j in i + 1..exact_vec.len() {
                    assert!(exact_vec[i] % exact_vec[j] == 0 || exact_vec[j] % exact_vec[i] == 0);
                }
            }
        }
        let exact_set: HashSet<i32> = exact_vec.iter().cloned().collect();
        assert_eq!(exact_set.len(), expect_size);
    }

    #[test]
    fn test_case1() {
        check(vec![1, 2, 3], 2);
    }

    #[test]
    fn test_case2() {
        check(vec![1, 2, 4, 8], 4);
    }

    #[test]
    fn test_case3() {
        check(vec![5, 9, 18, 54, 108, 540, 90, 180, 360, 720], 6);
    }

    #[test]
    fn test_empty() {
        check(Vec::new(), 0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::largest_divisible_subset(vec![2, 3, 99, 7]));
    }
}
