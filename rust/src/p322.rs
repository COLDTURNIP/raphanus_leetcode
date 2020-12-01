/*
Problem 322. Coin Change
========================

https://leetcode.com/problems/coin-change/

You are given coins of different denominations and a total amount of money amount. Write a function
to compute the fewest number of coins that you need to make up that amount. If that amount of money
cannot be made up by any combination of the coins, return -1.

You may assume that you have an infinite number of each kind of coin.

Example 1:

    Input: coins = [1,2,5], amount = 11
    Output: 3
    Explanation: 11 = 5 + 5 + 1

Example 2:

    Input: coins = [2], amount = 3
    Output: -1

Example 3:

    Input: coins = [1], amount = 0
    Output: 0

Example 4:

    Input: coins = [1], amount = 1
    Output: 1

Example 5:

    Input: coins = [1], amount = 2
    Output: 2

Constraints:

    - 1 <= coins.length <= 12
    - 1 <= coins[i] <= 231 - 1
    - 0 <= amount <= 104
*/

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount < 0 || coins.is_empty() {
            return if amount == 0 { 0 } else { -1 };
        }
        let mut cache = vec![std::i32::MAX; amount as usize + 1];
        cache[0] = 0;
        for n in 1..=amount {
            for c in coins.iter().filter(|&&c| n >= c) {
                let past = cache[(n - c) as usize];
                if past < std::i32::MAX {
                    let cnt = &mut cache[n as usize];
                    *cnt = (*cnt).min(1 + past);
                }
            }
        }
        if cache[amount as usize] <= amount {
            cache[amount as usize]
        } else {
            -1
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::coin_change(vec![1], 2), 2);
    }

    #[test]
    fn test_zero() {
        assert_eq!(Solution::coin_change(vec![5, 1, 8], 0), 0);
    }

    #[test]
    fn test_impossible() {
        assert_eq!(Solution::coin_change(vec![3, 9, 5], 4), -1);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::coin_change(vec![1, 2, 5], 99));
    }
}
