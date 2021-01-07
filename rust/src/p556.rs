/*
Problem 556. Next Greater Element III
=====================================

Given a positive integer n, find the smallest integer which has exactly the same digits existing in
the integer n and is greater in value than n. If no such positive integer exists, return -1.

Note that the returned integer should fit in 32-bit integer, if there is a valid answer but it does
not fit in 32-bit integer, return -1.

Example 1:

    Input: n = 12
    Output: 21

Example 2:

    Input: n = 21
    Output: -1

Constraints:

    - 1 <= n <= 2^31 - 1
*/

impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut digits: Vec<i32> = {
            let mut rest = n;
            std::iter::from_fn(|| {
                if rest == 0 {
                    None
                } else {
                    let d = rest % 10;
                    rest /= 10;
                    Some(d)
                }
            })
            .collect()
        };

        if let Some(idx) = (1..digits.len()).find(|&i| digits[i - 1] > digits[i]) {
            let target = digits[idx];
            digits[0..idx].reverse();
            let swap_i = (0..idx).find(|&i| digits[i] <= target).unwrap_or(idx) - 1;
            digits.swap(idx, swap_i);
            digits
                .iter()
                .rev()
                .fold(Some(0), |may_sum: Option<i32>, &n| {
                    may_sum?.checked_mul(10)?.checked_add(n)
                })
                .unwrap_or(-1)
        } else {
            -1
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(Solution::next_greater_element(12), 21);
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::next_greater_element(21), -1);
    }

    #[test]
    fn test_case3() {
        assert_eq!(Solution::next_greater_element(2147483486), -1);
    }

    #[test]
    fn test_case4() {
        assert_eq!(Solution::next_greater_element(12443322), 13222344);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::next_greater_element(23725));
    }
}
