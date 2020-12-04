/*
Problem 279. Perfect Squares
============================

https://leetcode.com/problems/perfect-squares/

Given a positive integer n, find the least number of perfect square numbers (for example, 1, 4, 9,
16, ...) which sum to n.

Example 1:

    Input: n = 12
    Output: 3
    Explanation: 12 = 4 + 4 + 4.

Example 2:

    Input: n = 13
    Output: 2
    Explanation: 13 = 4 + 9.
*/

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }
        let n = n as usize;
        let sqrs: Vec<usize> = (1..).map(|x| x * x).take_while(|&x| x <= n).collect();
        if sqrs.last().unwrap() == &n {
            return 1;
        }

        let mut ans = vec![0; n + 1];
        for n in 1..=n {
            // Note: It is faster for loop instead of iterator chaining on Leetcode.
            // Uncomment the following code in real world.
            //
            // let sqr_iter = sqrs.iter().take_while(|&&m| m <= n);
            // ans[n] = sqr_iter.map(|&s| 1 + ans[n - s]).min().unwrap_or(n);

            let mut ans_n = n;
            for &s in sqrs.iter() {
                if s > n {
                    break;
                }
                ans_n = ans_n.min(1 + ans[n - s]);
            }
            ans[n] = ans_n;
        }
        ans[n] as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case12() {
        assert_eq!(Solution::num_squares(12), 3);
    }

    #[test]
    fn test_case13() {
        assert_eq!(Solution::num_squares(13), 2);
    }

    #[test]
    fn test_zero() {
        assert_eq!(Solution::num_squares(0), 0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::num_squares(99));
    }
}
