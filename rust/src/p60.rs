/*
Problem 60. Permutation Sequence
================================

https://leetcode.com/problems/permutation-sequence/

The set [1,2,3,...,n] contains a total of n! unique permutations.

By listing and labeling all of the permutations in order, we get the following sequence for n = 3:

"123"
"132"
"213"
"231"
"312"
"321"
Given n and k, return the kth permutation sequence.

Note:

    Given n will be between 1 and 9 inclusive.
    Given k will be between 1 and n! inclusive.

Example 1:

Input: n = 3, k = 3
    Output: "213"
    Example 2:

Input: n = 4, k = 9
    Output: "2314"
*/

use std::collections::BTreeSet;

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        // fact: there are (n-i+1)! permutations for each value of digit i.
        let mut num_set = (1..=n).collect::<BTreeSet<i32>>();
        let mut picked = Vec::new();
        let mut rest = k as usize - 1;
        let mut fact: usize = (1..n as usize).product();
        while picked.len() < n as usize {
            let skip = rest / fact;
            rest %= fact;
            fact /= (num_set.len() - 1).max(1);
            let pick_digit = *num_set.iter().nth(skip).unwrap();
            picked.push(pick_digit.to_string());
            num_set.remove(&pick_digit);
        }
        picked.concat()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_3_3() {
        assert_eq!(Solution::get_permutation(3, 3), "213".to_owned());
    }

    #[test]
    fn test_4_9() {
        assert_eq!(Solution::get_permutation(4, 9), "2314".to_owned());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::get_permutation(10, 89));
    }
}
