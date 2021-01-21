/*
Problem 341. Flatten Nested List Iterator
=========================================

https://leetcode.com/problems/flatten-nested-list-iterator/

Given a nested list of integers, implement an iterator to flatten it.

Each element is either an integer, or a list -- whose elements may also be integers or other lists.

Example 1:

    Input: [[1,1],2,[1,1]]
    Output: [1,1,2,1,1]
    Explanation: By calling next repeatedly until hasNext returns false,
                 the order of elements returned by next should be: [1,1,2,1,1].

Example 2:

    Input: [1,[4,[6]]]
    Output: [1,4,6]
    Explanation: By calling next repeatedly until hasNext returns false,
                 the order of elements returned by next should be: [1,4,6].
*/

/**
 * Pre-defined by LeetCode
 */
#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

/**
 * Start of answer
 */
struct NestedIterator {
    iters: Vec<Box<dyn std::iter::Iterator<Item = NestedInteger>>>,
    next_entry: Option<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut ret = Self {
            iters: vec![Box::new(nested_list.into_iter())],
            next_entry: None,
        };
        ret.progress();
        ret
    }

    fn next(&mut self) -> i32 {
        let next = self.next_entry.unwrap_or(0);
        self.progress();
        next
    }

    fn has_next(&self) -> bool {
        !self.iters.is_empty()
    }

    fn progress(&mut self) {
        while !self.iters.is_empty() {
            let next_iter = if let Some(entry) = self.iters.last_mut().and_then(|iter| iter.next())
            {
                match entry {
                    NestedInteger::Int(n) => {
                        self.next_entry = Some(n);
                        return;
                    }
                    NestedInteger::List(next_list) => next_list.into_iter(),
                }
            } else {
                self.iters.pop();
                continue;
            };
            self.iters.push(Box::new(next_iter));
        }
    }
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::NestedInteger::{Int, List};
    use super::{NestedInteger, NestedIterator};

    fn check(input: Vec<NestedInteger>, expect: Vec<i32>) {
        let mut exact_iter = NestedIterator::new(input);
        let mut expect_iter = expect.into_iter();
        loop {
            let expect_entry = expect_iter.next();
            let exact_entry = if exact_iter.has_next() {
                Some(exact_iter.next())
            } else {
                None
            };
            assert_eq!(expect_entry, exact_entry);
            if expect_entry.is_none() && expect_entry.is_none() {
                break;
            }
        }
    }

    #[test]
    fn test_case1() {
        check(
            vec![
                List(vec![Int(1), Int(1)]),
                Int(2),
                List(vec![Int(1), Int(1)]),
            ],
            vec![1, 1, 2, 1, 1],
        );
    }

    #[test]
    fn test_case2() {
        check(
            vec![Int(1), List(vec![Int(4), List(vec![Int(6)])])],
            vec![1, 4, 6],
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            let mut iter = NestedIterator::new(vec![
                List(vec![Int(1), Int(2)]),
                Int(3),
                List(vec![Int(4), Int(5)]),
            ]);
            let mut cnt = 0;
            while iter.has_next() {
                if cnt > 5 {
                    break;
                }
                iter.next();
                cnt += 1;
            }
        });
    }
}
