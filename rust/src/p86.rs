/*
Problem 86. Partition List
==========================

https://leetcode.com/problems/partition-list/

Given a linked list and a value x, partition it such that all nodes less than x come before nodes
greater than or equal to x.

You should preserve the original relative order of the nodes in each of the two partitions.

Example:

    Input: head = 1->4->3->2->5->2, x = 3
    Output: 1->2->2->4->3->5
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
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut ans_head = ListNode { val: 0, next: None };
        let mut head_mod = &mut ans_head;
        let mut tail_head = ListNode { val: 0, next: None };
        let mut tail_mod = &mut tail_head;
        while let Some(node_ptr) = head.take() {
            if node_ptr.val < x {
                head_mod.next = Some(Box::new(ListNode {
                    val: node_ptr.val,
                    next: None,
                }));
                head_mod = head_mod.next.as_mut().unwrap();
            } else {
                tail_mod.next = Some(Box::new(ListNode {
                    val: node_ptr.val,
                    next: None,
                }));
                tail_mod = tail_mod.next.as_mut().unwrap();
            }
            head = node_ptr.next;
        }
        head_mod.next = tail_head.next;
        ans_head.next
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
    fn test_143252_3() {
        assert_eq!(
            Solution::partition(linkedlist![1, 4, 3, 2, 5, 2], 3),
            linkedlist![1, 2, 2, 4, 3, 5]
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::partition(None, 2), None);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::partition(linkedlist![1, 8, 9, 2, 7, 2, 3, 4, 5], 4));
    }
}
