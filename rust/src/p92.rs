/*
Problem 92. Reverse Linked List II
==================================

https://leetcode.com/problems/reverse-linked-list-ii/

Reverse a linked list from position m to n. Do it in one-pass.

Note: 1 ≤ m ≤ n ≤ length of list.

Example:

    Input: 1->2->3->4->5->NULL, m = 2, n = 4
    Output: 1->4->3->2->5->NULL
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
    pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        if m < 1 || m >= n {
            return head;
        }
        let mut pre_head = Box::new(ListNode { next: head, val: 0 });
        let mut tail = &mut pre_head;
        for _ in 1..m {
            if tail.next.is_some() {
                tail = tail.next.as_mut().unwrap();
            }
        }

        // reversing subseq
        unsafe {
            // need a cursor to point the tail node
            let mut tail_head = tail.next.take();
            if let Some(tail_ref) = tail_head.as_mut() {
                let mut rest = tail_ref.next.take();
                let tail_ptr: *mut ListNode = tail_ref.as_mut();
                for _ in m..n {
                    if let Some(mut node_ref) = rest {
                        rest = node_ref.next.take();
                        node_ref.next = tail_head;
                        tail_head = Some(node_ref);
                    }
                }
                (*tail_ptr).next = rest;
            }
            tail.next = tail_head;
        };

        pre_head.next
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
    fn test_case1() {
        assert_eq!(
            Solution::reverse_between(linkedlist![1, 2, 3, 4, 5], 2, 4),
            linkedlist![1, 4, 3, 2, 5]
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::reverse_between(linkedlist![1, 2, 3, 4, 5], 1, 5),
            linkedlist![5, 4, 3, 2, 1]
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::reverse_between(None, 0, 0), None);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::reverse_between(linkedlist![1, 2, 3, 3, 4, 4, 5], 2, 5));
    }
}
