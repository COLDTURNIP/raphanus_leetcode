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
            data: data,
            swaps: vec![0; size],
            i: 0,
        }
    }
}

impl<T> Iterator for PermutationIter<T>
where
T: Clone + Debug {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // Heap's algorithm
        // https://en.wikipedia.org/wiki/Heap's_algorithm
        if self.i > 0 {
            loop {
                if self.i >= self.swaps.len() { return None; }
                if self.swaps[self.i] < self.i { break; }
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

    #[test]
    fn test_i32() {
        let mut iter = PermutationIter::new(vec![1, 2, 3, 4]);
        assert_eq!(iter.next(), Some(vec![1, 2, 3, 4]));
        assert_eq!(iter.next(), Some(vec![2, 1, 3, 4]));
        assert_eq!(iter.next(), Some(vec![3, 1, 2, 4]));
        assert_eq!(iter.next(), Some(vec![1, 3, 2, 4]));
        assert_eq!(iter.next(), Some(vec![2, 3, 1, 4]));
        assert_eq!(iter.next(), Some(vec![3, 2, 1, 4]));
        assert_eq!(iter.next(), Some(vec![4, 2, 1, 3]));
        assert_eq!(iter.next(), Some(vec![2, 4, 1, 3]));
        assert_eq!(iter.next(), Some(vec![1, 4, 2, 3]));
        assert_eq!(iter.next(), Some(vec![4, 1, 2, 3]));
        assert_eq!(iter.next(), Some(vec![2, 1, 4, 3]));
        assert_eq!(iter.next(), Some(vec![1, 2, 4, 3]));
        assert_eq!(iter.next(), Some(vec![1, 3, 4, 2]));
        assert_eq!(iter.next(), Some(vec![3, 1, 4, 2]));
        assert_eq!(iter.next(), Some(vec![4, 1, 3, 2]));
        assert_eq!(iter.next(), Some(vec![1, 4, 3, 2]));
        assert_eq!(iter.next(), Some(vec![3, 4, 1, 2]));
        assert_eq!(iter.next(), Some(vec![4, 3, 1, 2]));
        assert_eq!(iter.next(), Some(vec![4, 3, 2, 1]));
        assert_eq!(iter.next(), Some(vec![3, 4, 2, 1]));
        assert_eq!(iter.next(), Some(vec![2, 4, 3, 1]));
        assert_eq!(iter.next(), Some(vec![4, 2, 3, 1]));
        assert_eq!(iter.next(), Some(vec![3, 2, 4, 1]));
        assert_eq!(iter.next(), Some(vec![2, 3, 4, 1]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_char() {
        let mut iter = PermutationIter::new("abcd".chars().collect());
        assert_eq!(iter.next(), Some(vec!['a', 'b', 'c', 'd']));
        assert_eq!(iter.next(), Some(vec!['b', 'a', 'c', 'd']));
        assert_eq!(iter.next(), Some(vec!['c', 'a', 'b', 'd']));
        assert_eq!(iter.next(), Some(vec!['a', 'c', 'b', 'd']));
        assert_eq!(iter.next(), Some(vec!['b', 'c', 'a', 'd']));
        assert_eq!(iter.next(), Some(vec!['c', 'b', 'a', 'd']));
        assert_eq!(iter.next(), Some(vec!['d', 'b', 'a', 'c']));
        assert_eq!(iter.next(), Some(vec!['b', 'd', 'a', 'c']));
        assert_eq!(iter.next(), Some(vec!['a', 'd', 'b', 'c']));
        assert_eq!(iter.next(), Some(vec!['d', 'a', 'b', 'c']));
        assert_eq!(iter.next(), Some(vec!['b', 'a', 'd', 'c']));
        assert_eq!(iter.next(), Some(vec!['a', 'b', 'd', 'c']));
        assert_eq!(iter.next(), Some(vec!['a', 'c', 'd', 'b']));
        assert_eq!(iter.next(), Some(vec!['c', 'a', 'd', 'b']));
        assert_eq!(iter.next(), Some(vec!['d', 'a', 'c', 'b']));
        assert_eq!(iter.next(), Some(vec!['a', 'd', 'c', 'b']));
        assert_eq!(iter.next(), Some(vec!['c', 'd', 'a', 'b']));
        assert_eq!(iter.next(), Some(vec!['d', 'c', 'a', 'b']));
        assert_eq!(iter.next(), Some(vec!['d', 'c', 'b', 'a']));
        assert_eq!(iter.next(), Some(vec!['c', 'd', 'b', 'a']));
        assert_eq!(iter.next(), Some(vec!['b', 'd', 'c', 'a']));
        assert_eq!(iter.next(), Some(vec!['d', 'b', 'c', 'a']));
        assert_eq!(iter.next(), Some(vec!['c', 'b', 'd', 'a']));
        assert_eq!(iter.next(), Some(vec!['b', 'c', 'd', 'a']));
        assert_eq!(iter.next(), None);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| PermutationIter::new((0..10).collect::<Vec<i32>>()).for_each(|_| {}));
    }
}
