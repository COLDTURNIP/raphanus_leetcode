/*
Permutation
===========
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
        //println!("assert_eq!(iter.next(), Some(vec!{:?}));", self);
        Some(self.data.clone())
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::PermutationIter;
    use std::collections::HashSet;

    #[test]
    fn test_i32() {
        let iter = PermutationIter::new(vec![1, 2, 3, 4]);
        assert_eq!(iter.collect::<HashSet<Vec<i32>>>().len(), 4 * 3 * 2);
    }

    #[test]
    fn test_char() {
        let iter = PermutationIter::new("abcd".chars().collect());
        assert_eq!(iter.collect::<HashSet<Vec<char>>>().len(), 4 * 3 * 2);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| PermutationIter::new((0..10).collect::<Vec<i32>>()).for_each(|_| {}));
    }
}
