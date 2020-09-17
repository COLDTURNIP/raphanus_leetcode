/*
Problem 78. Subsets
========================

https://leetcode.com/problems/subsets/

Given a set of distinct integers, nums, return all possible subsets (the power set).

Note: The solution set must not contain duplicate subsets.

Example:

    Input: nums = [1,2,3]
    Output:
    [
      [3],
      [1],
      [2],
      [1,2,3],
      [1,3],
      [2,3],
      [1,2],
      []
    ]
*/

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // refer to problem 77.

        let population = nums.len();
        let mut ans = Vec::new();
        ans.push(Vec::new());
        for pick in 1..=nums.len() {
            let mut state = (0..pick).collect::<Vec<usize>>();
            'iter_comb: loop {
                ans.push(state.iter().map(|&p| nums[p]).collect());
                let cursor = if let Some(c) = state
                    .iter()
                    .enumerate()
                    .rev()
                    .skip_while(|&(i, &val)| val >= population - pick + i)
                    .map(|pair| pair.0)
                    .next()
                {
                    c
                } else {
                    break 'iter_comb;
                };
                state[cursor] += 1;
                for i in cursor..pick - 1 {
                    state[i + 1] = state[i] + 1;
                }
            }
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

    fn check(input: Vec<i32>) {
        let possible_set_num = 1 << input.len();
        let mut result_set = HashSet::new();
        let input_set = input.clone().into_iter().collect::<HashSet<i32>>();
        for mut sub in Solution::subsets(input).into_iter() {
            sub.sort_unstable();
            assert!(sub.iter().all(|n| input_set.contains(n)));
            assert!(sub.clone().len() == sub.len());
            result_set.insert(sub);
        }
        assert_eq!(result_set.len(), possible_set_num);
    }

    #[test]
    fn test_123() {
        check(vec![1, 2, 3]);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::subsets(vec![1, 2, 3, 4, 5, 6, 7]));
    }
}
