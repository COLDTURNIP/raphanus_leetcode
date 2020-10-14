/*
Problem 1534. Count Good Triplets
=================================

https://leetcode.com/problems/count-good-triplets/

Given an array of integers arr, and three integers a, b and c. You need to find the number of good
triplets.

A triplet (arr[i], arr[j], arr[k]) is good if the following conditions are true:

    0 <= i < j < k < arr.length
    |arr[i] - arr[j]| <= a
    |arr[j] - arr[k]| <= b
    |arr[i] - arr[k]| <= c

Where |x| denotes the absolute value of x.

Return the number of good triplets.

Example 1:

    Input: arr = [3,0,1,1,9,7], a = 7, b = 2, c = 3
    Output: 4
    Explanation: There are 4 good triplets: [(3,0,1), (3,0,1), (3,1,1), (0,1,1)].

Example 2:

    Input: arr = [1,1,2,2,3], a = 0, b = 0, c = 1
    Output: 0
    Explanation: No triplet satisfies all conditions.

Constraints:

    - 3 <= arr.length <= 100
    - 0 <= arr[i] <= 1000
    - 0 <= a, b, c <= 1000
*/

use std::collections::BTreeMap;
use std::ops::Bound::Included;

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut pick_a: BTreeMap<i32, i32> = BTreeMap::new();
        let mut pick_ab: BTreeMap<(i32, i32), i32> = BTreeMap::new();
        let mut ans = 0;
        for n in arr.into_iter() {
            // handle k
            for (comb, cnt) in pick_ab.range((Included((n - c, n - b)), Included((n + c, n + b)))) {
                if (comb.0 - n).abs() <= c && (comb.1 - n).abs() <= b {
                    ans += cnt;
                }
            }
            // handle j
            for (&ni, cnt) in pick_a.range_mut((Included(n - a), Included(n + a))) {
                if (ni - n).abs() <= a {
                    *pick_ab.entry((ni, n)).or_insert(0) += *cnt;
                }
            }
            // handle i
            *pick_a.entry(n).or_insert(0) += 1;
        }
        ans
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_valid() {
        assert_eq!(
            Solution::count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3),
            4
        );
        assert_eq!(
            Solution::count_good_triplets(vec![7, 3, 7, 3, 12, 1, 12, 2, 3], 5, 8, 1),
            12
        );
    }

    #[test]
    fn test_invalid() {
        assert_eq!(
            Solution::count_good_triplets(vec![1, 1, 2, 2, 3], 0, 0, 1),
            0
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::count_good_triplets(Vec::new(), 0, 0, 0), 0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::count_good_triplets(vec![0, 2, 3, 4, 5, 2, 1, 0], 7, 2, 3));
    }
}
