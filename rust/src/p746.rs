/*
Problem 746. Min Cost Climbing Stairs
=====================================

https://leetcode.com/problems/min-cost-climbing-stairs/

On a staircase, the i-th step has some non-negative cost cost[i] assigned (0 indexed).

Once you pay the cost, you can either climb one or two steps. You need to find minimum cost to
reach the top of the floor, and you can either start from the step with index 0, or the step with
index 1.

Example 1:

    Input: cost = [10, 15, 20]
    Output: 15
    Explanation: Cheapest is start on cost[1], pay that cost and go to the top.

Example 2:

    Input: cost = [1, 100, 1, 1, 1, 100, 1, 1, 100, 1]
    Output: 6
    Explanation: Cheapest is start on cost[0], and only step on 1s, skipping cost[3].

Note:
    - cost will have a length in the range [2, 1000].
    - Every cost[i] will be an integer in the range [0, 999].
*/

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let (mut past1, mut past2) = (0, 0);
        for c in cost.into_iter() {
            past1 = c + past1.min(past2);
            std::mem::swap(&mut past1, &mut past2);
        }
        past1.min(past2)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15)
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        )
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]));
    }
}
