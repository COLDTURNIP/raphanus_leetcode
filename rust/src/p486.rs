/*
Problem 486. Predict the Winner
===============================

https://leetcode.com/problems/predict-the-winner/

Given an array of scores that are non-negative integers. Player 1 picks one of the numbers from
either end of the array followed by the player 2 and then player 1 and so on. Each time a player
picks a number, that number will not be available for the next player. This continues until all the
scores have been chosen. The player with the maximum score wins.

Given an array of scores, predict whether player 1 is the winner. You can assume each player plays
to maximize his score.

Example 1:

    Input: [1, 5, 2]
    Output: False
    Explanation: Initially, player 1 can choose between 1 and 2.
    If he chooses 2 (or 1), then player 2 can choose from 1 (or 2) and 5. If player 2 chooses 5,
    then player 1 will be left with 1 (or 2).
    So, final score of player 1 is 1 + 2 = 3, and player 2 is 5.
    Hence, player 1 will never be the winner and you need to return False.

Example 2:

    Input: [1, 5, 233, 7]
    Output: True
    Explanation: Player 1 first chooses 1. Then player 2 have to choose between 5 and 7. No matter
    which number player 2 choose, player 1 can choose 233.
    Finally, player 1 has more score (234) than player 2 (12), so you need to return True
    representing player1 can win.

Constraints:

    - 1 <= length of the array <= 20.
    - Any scores in the given array are non-negative integers and will not exceed 10,000,000.
    - If the scores of both players are equal, then player 1 is still the winner.
*/

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        assert!(!nums.is_empty());

        // core idea: In each turn, the socre of current player
        // score[i to j] = max nums[i]+score[i+1 to j] or nums[j]+score[i to j-1]

        let len = nums.len();
        // cur_score[i] in iteration j: final score played from nums[i to j]
        // last_score[i] in iteration j: final score played from nums[i to j-1]
        // positive: player1's score
        // negative: player2's score
        let mut cur_score: Vec<i32> = vec![0; len];
        let mut last_score: Vec<i32> = vec![0; len];
        let last_player_sign = if len % 2 == 0 { -1 } else { 1 };
        for end in 0..len {
            cur_score[end] = nums[end] * last_player_sign;
            let mut player_sign = -last_player_sign;
            for start in (0..end).rev() {
                let l_score = cur_score[start + 1] + nums[start] * player_sign;
                let r_score = last_score[start] + nums[end] * player_sign;
                cur_score[start] = if player_sign > 0 {
                    l_score.max(r_score)
                } else {
                    l_score.min(r_score)
                };
                player_sign = -player_sign;
            }
            std::mem::swap(&mut cur_score, &mut last_score);
        }
        last_score[0] >= 0
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert!(!Solution::predict_the_winner(vec![1, 5, 2]));
    }

    #[test]
    fn test_case2() {
        assert!(Solution::predict_the_winner(vec![1, 5, 233, 7]));
    }

    #[test]
    fn test_zero() {
        assert!(Solution::predict_the_winner(vec![0]));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::predict_the_winner(vec![1, 5, 233, 7]));
    }
}
