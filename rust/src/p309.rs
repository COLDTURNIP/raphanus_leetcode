/*
Problem 309. Best Time to Buy and Sell Stock with Cooldown
==========================================================

https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/

Say you have an array for which the ith element is the price of a given stock on day i.

Design an algorithm to find the maximum profit. You may complete as many transactions as you like
(ie, buy one and sell one share of the stock multiple times) with the following restrictions:

    - You may not engage in multiple transactions at the same time (ie, you must sell the stock
      before you buy again).
    - After you sell your stock, you cannot buy stock on next day. (ie, cooldown 1 day)

Example:

    Input: [1,2,3,0,2]
    Output: 3
    Explanation: transactions = [buy, sell, cooldown, buy, sell]
*/

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut bought = std::i32::MIN;
        let mut last_sold = 0;
        for price in prices {
            let last_sold_profit = profit;
            profit = profit.max(bought + price);
            bought = bought.max(last_sold - price);
            last_sold = last_sold_profit;
        }
        profit
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::max_profit(vec![2, 1, 4]), 3);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::max_profit(Vec::new()), 0);
        assert_eq!(Solution::max_profit(vec![999]), 0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::max_profit(vec![1, 2, 3, 0, 2]));
    }
}
