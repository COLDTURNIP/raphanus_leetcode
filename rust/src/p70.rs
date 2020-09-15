/*
Problem 70. Climbing Stairs
===========================

https://leetcode.com/problems/climbing-stairs/

You are climbing a stair case. It takes n steps to reach to the top.

Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?

Example 1:

    Input: 2
    Output: 2
    Explanation: There are two ways to climb to the top.
        1. 1 step + 1 step
        2. 2 steps

Example 2:

    Input: 3
    Output: 3
    Explanation: There are three ways to climb to the top.
        1. 1 step + 1 step + 1 step
        2. 1 step + 2 steps
        3. 2 steps + 1 step

Constraints:

    1 <= n <= 45
*/

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        (0..)
            .take_while(|n2| 2 * n2 <= n)
            .map(|n2| ((n - 2 * n2) as u64, n2 as u64))
            .map(|(n1, n2)| Self::choose_cnt(n1 + n2, n2))
            .sum::<u64>() as i32
    }

    // C(i, j)
    fn choose_cnt(i: u64, j: u64) -> u64 {
        let j = j.min(i - j);
        (1..=j).map(|bn| (i - bn + 1, bn)).fold(1, |acc, (an, bn)| {
            let (ran, rbn) = Self::reduction_fraction(an, bn);
            acc / rbn * ran
        })
    }

    fn reduction_fraction(a: u64, b: u64) -> (u64, u64) {
        let (mut m, mut n) = if a > b { (a, b) } else { (b, a) };
        while n != 0 {
            std::mem::swap(&mut m, &mut n);
            n %= m;
        }
        (a / m, b / m)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_2() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::climb_stairs(10));
    }
}
