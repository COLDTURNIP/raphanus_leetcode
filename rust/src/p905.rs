/*
Problem 905. Sort Array By Parity
=================================

https://leetcode.com/problems/sort-array-by-parity/

Given an array A of non-negative integers, return an array consisting of all the even elements of
A, followed by all the odd elements of A.

You may return any answer array that satisfies this condition.

Example 1:

    Input: [3,1,2,4]
    Output: [2,4,3,1]
    The outputs [4,2,3,1], [2,4,1,3], and [4,2,1,3] would also be accepted.

Note:

    1 <= A.length <= 5000
    0 <= A[i] <= 5000
*/

impl Solution {
    pub fn sort_array_by_parity(mut a: Vec<i32>) -> Vec<i32> {
        let len = a.len();
        let mut lb_iter = 0..len;
        let mut rb_iter = lb_iter.clone().rev();
        loop {
            let lb_found = lb_iter.find(|&i| a[i] & 1 == 1);
            let rb_found = rb_iter.find(|&i| a[i] & 1 == 0);
            match (lb_found, rb_found) {
                (Some(lb), Some(rb)) if lb < rb => a.swap(lb, rb),
                _ => break a,
            }
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;
    use std::collections::HashSet;

    fn check(input: Vec<i32>, expect: Vec<i32>) {
        let exact = Solution::sort_array_by_parity(input);
        let expect_head: HashSet<i32> = expect.iter().copied().take_while(|n| n % 2 == 0).collect();
        let expect_tail: HashSet<i32> = expect.iter().copied().skip_while(|n| n % 2 == 0).collect();
        let exact_head: HashSet<i32> = exact.iter().copied().take_while(|n| n % 2 == 0).collect();
        let exact_tail: HashSet<i32> = exact.iter().copied().skip_while(|n| n % 2 == 0).collect();
        assert_eq!(expect_head, exact_head);
        assert_eq!(expect_tail, exact_tail);
    }

    #[test]
    fn test_sort() {
        check(vec![3, 1, 2, 4], vec![2, 4, 3, 1]);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::sort_array_by_parity(vec![2, 7, 11, 15]));
    }
}
