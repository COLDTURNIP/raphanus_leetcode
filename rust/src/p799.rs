/*
Problem 799. Champagne Tower
============================

https://leetcode.com/problems/champagne-tower/

We stack glasses in a pyramid, where the first row has 1 glass, the second row has 2 glasses, and
so on until the 100th row.  Each glass holds one cup of champagne.

Then, some champagne is poured into the first glass at the top.  When the topmost glass is full,
any excess liquid poured will fall equally to the glass immediately to the left and right of it.
When those glasses become full, any excess champagne will fall equally to the left and right of
those glasses, and so on.  (A glass at the bottom row has its excess champagne fall on the floor.)

For example, after one cup of champagne is poured, the top most glass is full.  After two cups of
champagne are poured, the two glasses on the second row are half full.  After three cups of
champagne are poured, those two cups become full - there are 3 full glasses total now.  After four
cups of champagne are poured, the third row has the middle glass half full, and the two outside
glasses are a quarter full, as pictured below.

https://s3-lc-upload.s3.amazonaws.com/uploads/2018/03/09/tower.png

Now after pouring some non-negative integer cups of champagne, return how full the jth glass in the
ith row is (both i and j are 0-indexed.)

Example 1:

    Input: poured = 1, query_row = 1, query_glass = 1
    Output: 0.00000
    Explanation: We poured 1 cup of champange to the top glass of the tower (which is indexed as
    (0, 0)). There will be no excess liquid so all the glasses under the top glass will remain
    empty.

Example 2:

    Input: poured = 2, query_row = 1, query_glass = 1
    Output: 0.50000
    Explanation: We poured 2 cups of champange to the top glass of the tower (which is indexed as
    (0, 0)). There is one cup of excess liquid. The glass indexed as (1, 0) and the glass indexed
    as (1, 1) will share the excess liquid equally, and each will get half cup of champange.

Example 3:

    Input: poured = 100000009, query_row = 33, query_glass = 17
    Output: 1.00000


Constraints:

    - 0 <= poured <= 10^9
    - 0 <= query_glass <= query_row < 100
*/

impl Solution {
    fn champagne_amount(total_poured: f64, row: i32, glass: i32, amount: &mut [Vec<f64>]) -> f64 {
        if row < 0 || glass < 0 || glass > row {
            return 0.0;
        } else if amount[row as usize][glass as usize] >= 0.0 {
            return amount[row as usize][glass as usize];
        }
        let result = if row == 0 {
            total_poured
        } else {
            let left = (Self::champagne_amount(total_poured, row - 1, glass - 1, amount) - 1.0)
                .max(0.0)
                / 2.0;
            let right =
                (Self::champagne_amount(total_poured, row - 1, glass, amount) - 1.0).max(0.0) / 2.0;
            left + right
        };
        amount[row as usize][glass as usize] = result;
        result
    }

    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut amount = Vec::new();
        for r in 0..=query_row as usize {
            amount.push(vec![-1.0; r + 1]);
        }
        let ans = Self::champagne_amount(poured as f64, query_row, query_glass, &mut amount);
        ans.min(1.0)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    fn check(poured: i32, query_row: i32, query_glass: i32, expect: f64) {
        let exact = Solution::champagne_tower(poured, query_row, query_glass);
        assert!((exact - expect).abs() < std::f64::EPSILON);
    }

    #[test]
    fn test_case1() {
        check(1, 1, 1, 0.0)
    }

    #[test]
    fn test_case2() {
        check(2, 1, 1, 0.5)
    }

    #[test]
    fn test_case3() {
        check(100000009, 33, 17, 1.0)
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::champagne_tower(100000009, 33, 17));
    }
}
