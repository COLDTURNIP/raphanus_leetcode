/*
Problem 1550. Three Consecutive Odds
====================================

https://leetcode.com/problems/three-consecutive-odds/

Given an integer array arr, return true if there are three consecutive odd numbers in the array.
Otherwise, return false.

Example 1:

    Input: arr = [2,6,4,1]
    Output: false
    Explanation: There are no three consecutive odds.

Example 2:

    Input: arr = [1,2,34,3,4,5,7,23,12]
    Output: true
    Explanation: [5,7,23] are three consecutive odds.

Constraints:

    - 1 <= arr.length <= 1000
    - 1 <= arr[i] <= 1000
*/

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        arr.into_iter()
            .scan(0, |cnt, n| {
                if n & 1 == 1 {
                    *cnt += 1;
                } else {
                    *cnt = 0;
                }
                Some(*cnt)
            })
            .any(|n| n >= 3)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_found() {
        assert!(Solution::three_consecutive_odds(vec![
            1, 2, 34, 3, 4, 5, 7, 23, 12
        ]));
    }

    #[test]
    fn test_not_found() {
        assert!(!Solution::three_consecutive_odds(vec![7, 3, 4, 1]));
    }

    #[test]
    fn test_empty() {
        assert!(!Solution::three_consecutive_odds(Vec::new()));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::three_consecutive_odds(vec![1, 2, 34, 3, 4, 5, 7, 23, 12]));
    }
}
