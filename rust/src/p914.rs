/*
Problem 914. X of a Kind in a Deck of Cards
===========================================

https://leetcode.com/problems/x-of-a-kind-in-a-deck-of-cards/

In a deck of cards, each card has an integer written on it.

Return true if and only if you can choose X >= 2 such that it is possible to split the entire deck
into 1 or more groups of cards, where:

Each group has exactly X cards.
All the cards in each group have the same integer.

Example 1:

    Input: deck = [1,2,3,4,4,3,2,1]
    Output: true
    Explanation: Possible partition [1,1],[2,2],[3,3],[4,4].

Example 2:

    Input: deck = [1,1,1,2,2,2,3,3]
    Output: false´
    Explanation: No possible partition.

Example 3:

    Input: deck = [1]
    Output: false
    Explanation: No possible partition.

Example 4:

    Input: deck = [1,1]
    Output: true
    Explanation: Possible partition [1,1].

Example 5:

    Input: deck = [1,1,2,2,2,2]
    Output: true
    Explanation: Possible partition [1,1],[2,2],[2,2].

Constraints:

    - 1 <= deck.length <= 10^4
    - 0 <= deck[i] < 10^4
*/

use std::collections::HashMap;

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        if deck.is_empty() {
            return false;
        }
        let mut cnt = HashMap::new();
        for n in deck.into_iter() {
            *cnt.entry(n).or_insert(0) += 1;
        }
        cnt.values().fold(0, |gcd, &c| Self::gcd(gcd, c)) > 1
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }
        if b == 0 {
            return a;
        }
        loop {
            let tmp = a % b;
            if tmp == 0 {
                break b;
            }
            a = b;
            b = tmp;
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_pairs() {
        assert!(Solution::has_groups_size_x(vec![1, 2, 3, 4, 4, 3, 2, 1]));
        assert!(Solution::has_groups_size_x(vec![1, 1]));
        assert!(Solution::has_groups_size_x(vec![1, 2, 1, 2, 2, 2]));
    }

    #[test]
    fn test_impossible() {
        assert!(!Solution::has_groups_size_x(vec![1, 1, 1, 2, 2, 2, 3, 3]));
    }

    #[test]
    fn test_group_size_too_smal() {
        assert!(!Solution::has_groups_size_x(vec![1]));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::has_groups_size_x(vec![1, 1, 3, 4, 3, 4, 5, 5, 7, 9]));
    }
}
