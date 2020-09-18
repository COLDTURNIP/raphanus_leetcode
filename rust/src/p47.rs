/*
Problem 47. Permutations II
===========================

https://leetcode.com/problems/permutations-ii/

Given a collection of numbers that might contain duplicates, return all possible unique permutations.

Example:

    Input: [1,1,2]
    Output:
    [
      [1,1,2],
      [1,2,1],
      [2,1,1]
    ]
*/

use std::fmt::Debug;

#[derive(Debug)]
pub struct SortedPermutationIter<T> {
    data: Vec<T>,
    len: usize,
    init: bool,
}

impl<T> SortedPermutationIter<T>
where
    T: Clone + Debug + Eq + PartialEq + Ord + PartialOrd,
{
    pub fn new(mut data: Vec<T>) -> Self {
        data.sort_unstable();
        let len = data.len();
        Self {
            data,
            len,
            init: false,
        }
    }
}

impl<T> Iterator for SortedPermutationIter<T>
where
    T: Clone + Debug + Eq + PartialEq + Ord + PartialOrd,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // refer to p31: next permutation
        // step1. find the first decrease point as idx from right to left
        // step2. reverse data[idx..len], and find the lease number greater to idx-1
        if !self.init {
            self.init = true;
            Some(self.data.clone())
        } else if let Some(idx) = (1..self.len)
            .rev()
            .find(|&i| self.data[i] > self.data[i - 1])
        {
            let mut to_swap = self.len - 1;
            let (mut li, mut ri) = (idx, self.len - 1);
            while li <= ri {
                self.data.swap(li, ri);
                if self.data[li] > self.data[idx - 1] {
                    to_swap = to_swap.min(li);
                } else if self.data[ri] > self.data[idx - 1] {
                    to_swap = to_swap.min(ri);
                }
                li += 1;
                ri = ri.saturating_sub(1);
            }
            self.data.swap(idx - 1, to_swap);
            Some(self.data.clone())
        } else {
            None
        }
    }
}

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        SortedPermutationIter::new(nums).into_iter().collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;
    use std::collections::HashSet;

    fn check(input: Vec<i32>, expect: Vec<Vec<i32>>) {
        let exact = Solution::permute_unique(input)
            .into_iter()
            .collect::<HashSet<Vec<i32>>>();
        let expact = expect.into_iter().collect::<HashSet<Vec<i32>>>();
        assert_eq!(exact, expact);
    }

    #[test]
    fn test_perm_112() {
        check(
            vec![1, 1, 2],
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]],
        );
    }

    #[test]
    fn test_empty() {
        check(Vec::new(), vec![Vec::new()]);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::permute_unique(vec![1, 2, 3, 4, 5]));
    }
}
