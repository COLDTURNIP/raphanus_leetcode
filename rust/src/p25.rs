/*
24. Swap Nodes in Pairs
=======================

https://leetcode.com/problems/swap-nodes-in-pairs/

Given a linked list, reverse the nodes of a linked list k at a time and return its modified list.

k is a positive integer and is less than or equal to the length of the linked list. If the number
of nodes is not a multiple of k then left-out nodes in the end should remain as it is.

Example:

    Given this linked list: 1->2->3->4->5

    For k = 2, you should return: 2->1->4->3->5

    For k = 3, you should return: 3->2->1->4->5

Note:

Only constant extra memory is allowed.
You may not alter the values in the list's nodes, only nodes itself may be changed.
*/

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

use std::mem;

type Link = Option<Box<ListNode>>;

struct List {
    head: Link,
}

impl List {
    fn from_link(head: Link) -> List {
        List { head: head }
    }

    fn take(&mut self) -> Link {
        self.head.take()
    }

    fn reverse(&mut self) {
        let mut prev = None;
        let mut cur = self.head.take();
        while let Some(mut cur_inner) = cur.take() {
            let next = cur_inner.next.take();
            cur_inner.next = prev.take();
            prev = Some(cur_inner);
            cur = next;
        }
        self.head = prev.take();
    }

    fn split_at(&mut self, n: usize) -> List {
        if n == 0 {
            return List { head: self.take() }
        }
        let mut cur = self.head.as_mut().take();
        for _ in 0..n {
            match cur {
                None => break,
                Some(cur_ref) => {
                    cur = cur_ref.next.as_mut().take();
                }
            }
        }
        // List::from_link(cur.take())
        List{ head: None }
    }
}

impl Solution {

    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        None
    }

    fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list = List::from_link(head);
        list.reverse();
        list.take()
    }
}

use crate::ListNode;
pub struct Solution;

#[cfg(test)]
mod test {
    use crate::linkedlist;
    use super::Solution;

    #[test]
    fn test_reverse_k_group() {
        assert_eq!(Solution::reverse_k_group(linkedlist![1, 2, 3, 4], 2), linkedlist![2, 1, 4, 3]);
        assert_eq!(Solution::reverse_k_group(linkedlist![1, 2, 3, 4, 5], 2), linkedlist![2, 1, 4, 3, 5]);
        assert_eq!(Solution::reverse_k_group(linkedlist![1, 2, 3, 4, 5], 3), linkedlist![3, 2, 1, 4, 5]);
        assert_eq!(Solution::reverse_k_group(linkedlist![], 4), linkedlist![]);
    }

    #[test]
    fn test_reverse_list() {
        assert_eq!(Solution::reverse_list(linkedlist![1, 2, 3, 4]), linkedlist![4, 3, 2, 1]);
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use crate::linkedlist;
    use super::Solution;

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::reverse_k_group(linkedlist![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4));
    }
}
