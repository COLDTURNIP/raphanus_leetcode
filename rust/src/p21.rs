/*
Problem 21. Merge Two Sorted Lists
==================================

https://leetcode.com/problems/merge-two-sorted-lists/

Merge two sorted linked lists and return it as a new sorted list. The new list should be made by
splicing together the nodes of the first two lists.

Example:

    Input: 1->2->4, 1->3->4
    Output: 1->1->2->3->4->4
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

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        fn new_node(val: i32) -> Option<Box<ListNode>> {
            Some(Box::new(ListNode { val, next: None }))
        }

        let (mut l1, mut l2) = (l1, l2);
        let mut head = Some(Box::new(ListNode { val: 0, next: None }));
        let mut prev = &mut head;
        loop {
            match (l1.as_mut(), l2.as_mut()) {
                (Some(n1), Some(n2)) => {
                    if n1.val < n2.val {
                        if let Some(pn) = prev.as_mut() {
                            pn.next = new_node(n1.val);
                        }
                        l1 = n1.next.take();
                    } else {
                        if let Some(pn) = prev.as_mut() {
                            pn.next = new_node(n2.val);
                        }
                        l2 = n2.next.take();
                    }
                    prev = &mut prev.as_mut().unwrap().next;
                }
                (Some(n), None) | (None, Some(n)) => {
                    if let Some(pn) = prev.as_mut() {
                        pn.next = new_node(n.val);
                    }
                    l1 = n.next.take();
                    l2 = None;
                    prev = &mut prev.as_mut().unwrap().next;
                }
                _ => break,
            }
        }
        head.unwrap().next
    }
}

pub struct Solution;
use leetcode_prelude::ListNode;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;
    use leetcode_prelude::linkedlist;

    #[test]
    fn test_merge() {
        assert_eq!(
            Solution::merge_two_lists(linkedlist![1, 2, 4], linkedlist![1, 3, 4]),
            linkedlist![1, 1, 2, 3, 4, 4]
        );
    }

    #[test]
    fn test_single_empty() {
        assert_eq!(
            Solution::merge_two_lists(None, linkedlist![1, 2, 3]),
            linkedlist![1, 2, 3]
        );
        assert_eq!(
            Solution::merge_two_lists(linkedlist![1, 2, 3, 4], None),
            linkedlist![1, 2, 3, 4]
        );
    }

    #[test]
    fn test_double_empty() {
        assert_eq!(Solution::merge_two_lists(None, None), None);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::merge_two_lists(None, None));
    }
}
