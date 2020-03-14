/*
122. Best Time to Buy and Sell Stock II
=======================================

https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/

Say you have an array for which the ith element is the price of a given stock on day i.

Design an algorithm to find the maximum profit. You may complete as many transactions as you like
(i.e., buy one and sell one share of the stock multiple times).

Note: You may not engage in multiple transactions at the same time (i.e., you must sell the stock
before you buy again).

Example 1:

Input: [7,1,5,3,6,4]
Output: 7
Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5),
             profit = 5-1 = 4.
             Then buy on day 4 (price = 3) and sell on day 5 (price = 6),
             profit = 6-3 = 3.
Example 2:

Input: [1,2,3,4,5]
Output: 4
Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5),
             profit = 5-1 = 4.
             Note that you cannot buy on day 1, buy on day 2 and sell them later, as you are
             engaging multiple transactions at the same time. You must sell before buying again.

*/

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        return internal::trade_loop(&prices);
    }

}

pub struct Solution;

mod internal {
    pub fn trade_loop(prices: &[i32]) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        enum State { Buy, Sell, }
        let mut sum = 0;
        let mut state = State::Buy;
        let total_prices = prices.len();
        for (i, &cur) in prices.iter().enumerate() {
            match state {
                State::Buy => {
                    if i < prices.len()-1 && cur < prices[i+1] {
                        sum -= cur;
                        state = State::Sell;
                    }
                },
                State::Sell => {
                    if i == total_prices-1 || cur > prices[i+1] {
                        sum += cur;
                        state = State::Buy;
                    }
                }
            }
        }
        return sum;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::max_profit(vec![1, 2]), 1);
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use super::Solution;

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
    }
}
