/*
Problem 983. Minimum Cost For Tickets
=====================================

https://leetcode.com/problems/minimum-cost-for-tickets/

In a country popular for train travel, you have planned some train travelling one year in advance.
The days of the year that you will travel is given as an array days.  Each day is an integer from 1
to 365.

Train tickets are sold in 3 different ways:

    - a 1-day pass is sold for costs[0] dollars;
    - a 7-day pass is sold for costs[1] dollars;
    - a 30-day pass is sold for costs[2] dollars.

The passes allow that many days of consecutive travel.  For example, if we get a 7-day pass on day
2, then we can travel for 7 days: day 2, 3, 4, 5, 6, 7, and 8.

Return the minimum number of dollars you need to travel every day in the given list of days.

Example 1:

    Input: days = [1,4,6,7,8,20], costs = [2,7,15]
    Output: 11
    Explanation:
    For example, here is one way to buy passes that lets you travel your travel plan:
    On day 1, you bought a 1-day pass for costs[0] = $2, which covered day 1.
    On day 3, you bought a 7-day pass for costs[1] = $7, which covered days 3, 4, ..., 9.
    On day 20, you bought a 1-day pass for costs[0] = $2, which covered day 20.
    In total you spent $11 and covered all the days of your travel.

Example 2:

    Input: days = [1,2,3,4,5,6,7,8,9,10,30,31], costs = [2,7,15]
    Output: 17
    Explanation:
    For example, here is one way to buy passes that lets you travel your travel plan:
    On day 1, you bought a 30-day pass for costs[2] = $15 which covered days 1, 2, ..., 30.
    On day 31, you bought a 1-day pass for costs[0] = $2 which covered day 31.
    In total you spent $17 and covered all the days of your travel.

Note:

    - 1 <= days.length <= 365
    - 1 <= days[i] <= 365
    - days is in strictly increasing order.
    - costs.length == 3
    - 1 <= costs[i] <= 1000
*/

impl Solution {
    fn mincost_tickets_impl(
        days: &[i32],
        costs: &[i32],
        min_cost: &mut [i32],
        start: usize,
    ) -> i32 {
        match min_cost.get(start) {
            Some(&cost) if cost > 0 => return cost,
            None => return 0,
            _ => (),
        };
        let cur_day = days[start];
        let next7 = days
            .iter()
            .enumerate()
            .skip(start + 1)
            .find_map(|(i, &d)| if d >= cur_day + 7 { Some(i) } else { None })
            .unwrap_or(days.len());
        let next30 = days
            .iter()
            .enumerate()
            .skip(start + 1)
            .find_map(|(i, &d)| if d >= cur_day + 30 { Some(i) } else { None })
            .unwrap_or(days.len());
        let day1_cost = costs[0] + Self::mincost_tickets_impl(days, costs, min_cost, start + 1);
        let day7_cost = costs[1] + Self::mincost_tickets_impl(days, costs, min_cost, next7);
        let day30_cost = costs[2] + Self::mincost_tickets_impl(days, costs, min_cost, next30);
        let cost = day1_cost.min(day7_cost).min(day30_cost);
        min_cost[start] = cost;
        cost
    }

    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        Self::mincost_tickets_impl(&days, &costs, &mut vec![0; days.len()], 0)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15]),
            11
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15]),
            17
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15])
        });
    }
}
