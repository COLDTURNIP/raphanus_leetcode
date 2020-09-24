/*
Problem 119. Pascal's Triangle II
=================================

https://leetcode.com/problems/pascals-triangle-ii/

Given an integer rowIndex, return the rowIndexth row of the Pascal's triangle.

Notice that the row index starts from 0.

In Pascal's triangle, each number is the sum of the two numbers directly above it.

Follow up:

Could you optimize your algorithm to use only O(k) extra space?

Follow up:

Could you optimize your algorithm to use only O(k) extra space?


Example 1:

    Input: rowIndex = 3
    Output: [1,3,3,1]

Example 2:

    Input: rowIndex = 0
    Output: [1]

Example 3:

    Input: rowIndex = 1
    Output: [1,1]

Constraints:

    0 <= rowIndex <= 40
*/

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        if row_index < 0 {
            return ans;
        }
        ans.push(1);
        let m = row_index as usize;
        // C(m, n+1) = C(m, n) * (m-n+1) / (n+1)
        for n in 1..=m {
            ans.push((ans[n - 1] as usize * (m - n + 1) / n) as i32);
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
    fn test_5() {
        assert_eq!(Solution::get_row(5), vec![1, 5, 10, 10, 5, 1]);
    }

    #[test]
    fn test_1() {
        assert_eq!(Solution::get_row(1), vec![1, 1]);
    }

    #[test]
    fn test_zero() {
        assert_eq!(Solution::get_row(0), vec![1]);
        assert_eq!(Solution::get_row(-1), Vec::<i32>::new());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::get_row(10));
    }
}
