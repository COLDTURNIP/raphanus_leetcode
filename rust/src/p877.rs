/*
Problem 877. Stone Game
=======================

https://leetcode.com/problems/stone-game/

Alex and Lee play a game with piles of stones. There are an even number of piles arranged in a row,
and each pile has a positive integer number of stones piles[i].

The objective of the game is to end with the most stones. The total number of stones is odd, so
there are no ties.

Alex and Lee take turns, with Alex starting first. Each turn, a player takes the entire pile of
stones from either the beginning or the end of the row.  This continues until there are no more
piles left, at which point the person with the most stones wins.

Assuming Alex and Lee play optimally, return True if and only if Alex wins the game.

Example 1:

    Input: piles = [5,3,4,5]
    Output: true
    Explanation:
        Alex starts first, and can only take the first 5 or the last 5.
        Say he takes the first 5, so that the row becomes [3, 4, 5].
        If Lee takes 3, then the board is [4, 5], and Alex takes 5 to win with 10 points.
        If Lee takes the last 5, then the board is [3, 4], and Alex takes 4 to win with 9 points.
        This demonstrated that taking the first 5 was a winning move for Alex, so we return true.

Constraints:

    - 2 <= piles.length <= 500
    - piles.length is even.
    - 1 <= piles[i] <= 500
    - sum(piles) is odd.
*/

use std::collections::HashMap;

impl Solution {
    fn is_a_win(
        a_score: i32,
        b_score: i32,
        piles: &[i32],
        choose: (usize, usize),
        cache: &mut HashMap<(usize, usize), bool>,
    ) -> bool {
        if let Some(&result) = cache.get(&choose) {
            return result;
        }
        let (left, right) = (choose.0, choose.1);
        let result = if left == right {
            // the last turn
            a_score > b_score + piles[choose.0]
        } else if (right - left) & 1 == 1 {
            // a's turn
            let result_l = Solution::is_a_win(
                a_score + piles[left],
                b_score,
                piles,
                (left + 1, right),
                cache,
            );
            let result_r = Solution::is_a_win(
                a_score + piles[right],
                b_score,
                piles,
                (left, right - 1),
                cache,
            );
            result_l || result_r
        } else {
            // b's turn
            let result_l = Solution::is_a_win(
                a_score,
                b_score + piles[left],
                piles,
                (left + 1, right),
                cache,
            );
            let result_r = Solution::is_a_win(
                a_score,
                b_score + piles[right],
                piles,
                (left, right - 1),
                cache,
            );
            result_l || result_r
        };
        cache.insert(choose, result);
        result
    }

    pub fn stone_game(piles: Vec<i32>) -> bool {
        // In fact, the answer always true. Just return true and get 100%.
        Solution::is_a_win(0, 0, &piles, (0, piles.len() - 1), &mut HashMap::new())
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_win() {
        assert!(Solution::stone_game(vec![5, 3, 4, 5]));
    }

    #[test]
    fn test_may_lose() {
        assert!(Solution::stone_game(vec![3, 7, 10, 5]));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::stone_game(vec![5, 3, 4, 5]));
    }
}
