/*
24. Swap Nodes in Pairs
=======================

https://leetcode.com/problems/swap-nodes-in-pairs/

Given a linked list, swap every two adjacent nodes and return its head.

You may not modify the values in the list's nodes, only nodes itself may be changed.

 

Example:

    Given 1->2->3->4, you should return the list as 2->1->4->3.
*/

//
// LeetCode Prelude
//
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut base = Some(Box::new(ListNode { val: 0, next: head }));
        let mut cur = base.as_mut();

        fn take_next(node: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            node.as_mut().unwrap().next.take()
        }
        fn set_next(node: &mut Option<Box<ListNode>>, target: Option<Box<ListNode>>) {
            node.as_mut().unwrap().next = target;
        }
        loop {
            let mut fst = match cur.as_mut() {
                Some(inner) => inner.next.take(),
                _ => None,
            };
            if fst.is_none() {
                break;
            }
            let mut sec = match fst.as_mut() {
                Some(inner) => inner.next.take(),
                _ => None,
            };
            match sec {
                Some(_) => {
                    let cur_next = take_next(&mut sec);
                    set_next(&mut fst, cur_next);
                    set_next(&mut sec, fst);
                    cur.as_mut().unwrap().next = sec;
                    cur = cur.unwrap().next.as_mut().unwrap().next.as_mut();
                },
                None => {
                    if let Some(inner) = cur.as_mut() {
                        inner.next = fst;
                    }
                    break;
                },
            };
        }
        match base { Some(base) => base.next, None => None }
    }
}

use crate::ListNode;
pub struct Solution;

#[cfg(test)]
mod test {
    use crate::linkedlist;
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::swap_pairs(linkedlist![1, 2, 3, 4]), linkedlist![2, 1, 4, 3]);
        assert_eq!(Solution::swap_pairs(linkedlist![1, 2, 3]), linkedlist![2, 1, 3]);
        assert_eq!(Solution::swap_pairs(linkedlist![]), linkedlist![]);
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use crate::linkedlist;
    use super::Solution;

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::swap_pairs(linkedlist![1, 2, 3, 4]));
    }
}
