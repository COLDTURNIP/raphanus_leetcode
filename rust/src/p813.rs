/*
Problem 813. Largest Sum of Averages
====================================

https://leetcode.com/problems/largest-sum-of-averages/

We partition a row of numbers A into at most K adjacent (non-empty) groups, then our score is the
sum of the average of each group. What is the largest score we can achieve?

Note that our partition must use every number in A, and that scores are not necessarily integers.

Example:

    Input:
    A = [9,1,2,3,9]
    K = 3
    Output: 20
    Explanation:
    The best choice is to partition A into [9], [1, 2, 3], [9]. The answer is 9 + (1 + 2 + 3) / 3 + 9 = 20.
    We could have also partitioned A into [9, 1], [2], [3, 9], for example.
    That partition would lead to a score of 5 + 2 + 6 = 13, which is worse.

Note:

    - 1 <= A.length <= 100.
    - 1 <= A[i] <= 10000.
    - 1 <= K <= A.length.
    - Answers within 10^-6 of the correct answer will be accepted as correct.
*/

use std::collections::HashMap;

impl Solution {
    fn largest_sum_of_avg_impl(
        a: &[i32],
        k: usize,
        start: usize,
        cache: &mut HashMap<usize, Vec<f64>>,
    ) -> f64 {
        let len = a.len();
        if let Some(k1_cache) = cache.get(&k) {
            match k1_cache.get(start) {
                Some(&ans) if ans > 0.0 => return ans,
                None => return 0.0,
                _ => (),
            }
        } else {
            cache.insert(k, vec![0.0; len]);
        }
        // cache[k] means the largest ans when split a from index start to the end into k groups
        if k <= 1 {
            let k1_cache = cache.get_mut(&k).unwrap();
            let mut sum = 0;
            for i in (0..len).rev() {
                sum += a[i];
                k1_cache[i] = sum as f64 / (len - i) as f64;
            }
            k1_cache[start]
        } else {
            let mut ans: f64 = 0.0;
            for end in start + 1..=len - (k - 1) {
                let sum = a[start..end].iter().sum::<i32>() as f64;
                let avg = sum / (end - start) as f64;
                let sub_avg = Solution::largest_sum_of_avg_impl(a, k - 1, end, cache);
                ans = ans.max(avg + sub_avg);
            }
            cache.get_mut(&k).unwrap()[start] = ans;
            ans
        }
    }

    pub fn largest_sum_of_averages(a: Vec<i32>, k: i32) -> f64 {
        Self::largest_sum_of_avg_impl(&a, k as usize, 0, &mut HashMap::new())
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    fn check(input_a: Vec<i32>, input_k: i32, expect: f64) {
        let exact = Solution::largest_sum_of_averages(input_a, input_k);
        assert!((exact - expect).abs() < std::f64::EPSILON);
    }

    #[test]
    fn test_case1() {
        check(vec![9, 1, 2, 3, 9], 3, 20.0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::largest_sum_of_averages(vec![9, 1, 2, 3, 9], 3));
    }
}
