/*
Problem 46. Permutations
========================

https://leetcode.com/problems/permutations/

Given a collection of distinct integers, return all possible permutations.

Example:

    Input: [1,2,3]
    Output:
    [
      [1,2,3],
      [1,3,2],
      [2,1,3],
      [2,3,1],
      [3,1,2],
      [3,2,1]
    ]
*/

use std::fmt::Debug;

#[derive(Debug)]
pub struct PermutationIter<T> {
    data: Vec<T>,
    swaps: Vec<usize>,
    i: usize,
}

impl<T> PermutationIter<T> {
    pub fn new(data: Vec<T>) -> Self {
        let size = data.len();
        Self {
            data,
            swaps: vec![0; size],
            i: 0,
        }
    }
}

impl<T> Iterator for PermutationIter<T>
where
    T: Clone + Debug,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // Heap's algorithm
        // https://en.wikipedia.org/wiki/Heap's_algorithm
        if self.i > 0 {
            loop {
                if self.i >= self.swaps.len() {
                    return None;
                }
                if self.swaps[self.i] < self.i {
                    break;
                }
                self.swaps[self.i] = 0;
                self.i += 1;
            }
            self.data.swap(self.i, (self.i & 1) * self.swaps[self.i]);
            self.swaps[self.i] += 1;
        }
        self.i = 1;
        Some(self.data.clone())
    }
}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        PermutationIter::new(nums).into_iter().collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;
    use std::collections::HashSet;

    fn check(input: Vec<i32>, expect: Vec<Vec<i32>>) {
        let exact = Solution::permute(input)
            .into_iter()
            .collect::<HashSet<Vec<i32>>>();
        let expact = expect.into_iter().collect::<HashSet<Vec<i32>>>();
        assert_eq!(exact, expact);
    }

    #[test]
    fn test_perm3() {
        check(
            vec![1, 2, 3],
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1],
            ],
        );
    }

    #[test]
    fn test_empty() {
        check(Vec::new(), vec![Vec::new()]);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::permute(vec![1, 2, 3, 4, 5]));
    }
}
