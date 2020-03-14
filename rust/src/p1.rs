/*
Problem 18. 4Sum
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

        for (ia, a) in nums.iter().enumerate() {
            let b = target - a;
            match seen.get(&b) {
                Some(&ib) => return vec![ia as i32, ib as i32],
                None => seen.insert(a, ia),
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
