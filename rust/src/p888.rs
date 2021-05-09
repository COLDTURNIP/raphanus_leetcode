/*
Problem 888. Fair Candy Swap
============================

https://leetcode.com/problems/fair-candy-swap/

Alice and Bob have candy bars of different sizes: A[i] is the size of the i-th bar of candy that
Alice has, and B[j] is the size of the j-th bar of candy that Bob has.

Since they are friends, they would like to exchange one candy bar each so that after the exchange,
they both have the same total amount of candy.  (The total amount of candy a person has is the sum
of the sizes of candy bars they have.)

Return an integer array ans where ans[0] is the size of the candy bar that Alice must exchange, and
ans[1] is the size of the candy bar that Bob must exchange.

If there are multiple answers, you may return any one of them.  It is guaranteed an answer exists.

Example 1:

    Input: A = [1,1], B = [2,2]
    Output: [1,2]

Example 2:

    Input: A = [1,2], B = [2,3]
    Output: [1,2]

Example 3:

    Input: A = [2], B = [1,3]
    Output: [2,3]

Example 4:

    Input: A = [1,2,5], B = [2,4]
    Output: [5,4]

Note:

    - 1 <= A.length <= 10000
    - 1 <= B.length <= 10000
    - 1 <= A[i] <= 100000
    - 1 <= B[i] <= 100000
    - It is guaranteed that Alice and Bob have different total amounts of candy.
    - It is guaranteed there exists an answer.
*/

use std::collections::HashSet;

impl Solution {
    pub fn fair_candy_swap(a: Vec<i32>, mut b: Vec<i32>) -> Vec<i32> {
        let (set_a, sum_a) = {
            let mut set = HashSet::new();
            let mut sum = 0;
            for n in a.into_iter() {
                set.insert(n);
                sum += n;
            }
            (set, sum)
        };
        b.sort_unstable();
        let sum_b = b.iter().sum::<i32>();
        let diff = (sum_b - sum_a) / 2;
        for na in set_a.into_iter() {
            if let Ok(ib) = b.binary_search(&(na + diff)) {
                return vec![na, b[ib]];
            }
        }
        unreachable!();
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    fn check(list_a: &[i32], list_b: &[i32]) {
        let (sum_a, sum_b) = (list_a.iter().sum::<i32>(), list_b.iter().sum::<i32>());
        let ans = Solution::fair_candy_swap(list_a.into(), list_b.into());
        assert_eq!(sum_a - ans[0] + ans[1], sum_b - ans[1] + ans[0]);
    }

    #[test]
    fn test_exchange() {
        check(&[1, 1], &[2, 2]);
        check(&[1, 2], &[2, 3]);
        check(&[2], &[1, 3]);
        check(&[1, 2, 5], &[2, 4]);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::fair_candy_swap(vec![1, 2, 5], vec![2, 4]));
    }
}
