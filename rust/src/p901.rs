/*
Problem 901. Online Stock Span
==============================

https://leetcode.com/problems/online-stock-span/

Write a class StockSpanner which collects daily price quotes for some stock, and returns the span
of that stock's price for the current day.

The span of the stock's price today is defined as the maximum number of consecutive days (starting
from today and going backwards) for which the price of the stock was less than or equal to today's
price.

For example, if the price of a stock over the next 7 days were [100, 80, 60, 70, 60, 75, 85], then
the stock spans would be [1, 1, 1, 2, 1, 4, 6].

Example 1:

    Input: ["StockSpanner","next","next","next","next","next","next","next"],
           [[],[100],[80],[60],[70],[60],[75],[85]]
    Output: [null,1,1,1,2,1,4,6]
    Explanation:
    First, S = StockSpanner() is initialized.  Then:
    S.next(100) is called and returns 1,
    S.next(80) is called and returns 1,
    S.next(60) is called and returns 1,
    S.next(70) is called and returns 2,
    S.next(60) is called and returns 1,
    S.next(75) is called and returns 4,
    S.next(85) is called and returns 6.

    Note that (for example) S.next(75) returned 4, because the last 4 prices
    (including today's price of 75) were less than or equal to today's price.

Note:

    - Calls to StockSpanner.next(int price) will have 1 <= price <= 10^5.
    - There will be at most 10000 calls to StockSpanner.next per test case.
    - There will be at most 150000 calls to StockSpanner.next across all test cases.
    - The total time limit for this problem has been reduced by 75% for C++, and 50% for all other
      languages.
*/

/**
 * Start of answer
 */
#[derive(Default)]
struct StockSpanner {
    history: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {
    fn new() -> Self {
        Self::default()
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut span = 1;
        let mut history_len = self.history.len();
        for rec_i in (0..history_len).rev() {
            if self.history[rec_i].0 <= price {
                span += self.history[rec_i].1;
                history_len = rec_i;
            }
        }
        self.history.truncate(history_len);
        self.history.push((price, span));
        span
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::StockSpanner;

    fn check(input: Vec<i32>, expect: Vec<i32>) {
        let mut spanner = StockSpanner::new();
        assert_eq!(input.len(), expect.len());
        for (input_price, expect_price) in input.into_iter().zip(expect) {
            assert_eq!(spanner.next(input_price), expect_price);
        }
    }

    #[test]
    fn test_case1() {
        check(vec![100, 80, 60, 70, 60, 75, 85], vec![1, 1, 1, 2, 1, 4, 6]);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            let mut spanner = StockSpanner::new();
            for &price in [100, 80, 60, 70, 60, 75, 85].iter() {
                spanner.next(price);
            }
        });
    }
}
