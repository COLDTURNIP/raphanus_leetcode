/*
Problem 1. Two Sum
=======================================

https://leetcode.com/problems/two-sum/

Given nums = [2, 7, 11, 15], target = 9,

Because nums[0] + nums[1] = 2 + 7 = 9,
return [0, 1].

*/
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::with_capacity(nums.len());

        for (cur_i, cur_v) in nums.iter().enumerate() {
            let rest = target - cur_v;
            match seen.get(&rest) {
                Some(&seen_i) => return vec![seen_i as i32, cur_i as i32],
                None => seen.insert(cur_v, cur_i),
            };
        }
        unreachable!();
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        let question = vec![2, 7, 11, 15];
        assert_eq!(Solution::two_sum(question, 9), vec![0, 1]);
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use super::Solution;

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::two_sum(vec![1, 2, 3, 4], 5))
    }
}
