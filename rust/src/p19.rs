/*
Problem 19. Remove Nth Node From End of List
============================================

https://leetcode.com/problems/remove-nth-node-from-end-of-list/

Given the head of a linked list, remove the nth node from the end of the list and return its head.

Follow up: Could you do this in one pass?

Example 1:

    https://assets.leetcode.com/uploads/2020/10/03/remove_ex1.jpg

    Input: head = [1,2,3,4,5], n = 2
    Output: [1,2,3,5]

Example 2:

    Input: head = [1], n = 1
    Output: []

Example 3:

    Input: head = [1,2], n = 1
    Output: [1]

Constraints:

The number of nodes in the list is sz.
    - 1 <= sz <= 30
    - 0 <= Node.val <= 100
    - 1 <= n <= sz
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut pre_head = ListNode { next: head, val: 0 };
        let pre_rm_node = unsafe {
            // two cursors for performance
            let mut probe: *const ListNode = &pre_head;
            for _step in 0..n {
                if let Some(node_ref) = (*probe).next.as_ref() {
                    probe = node_ref.as_ref();
                } else {
                    return pre_head.next;
                }
            }
            let mut pre_rm_node: *mut ListNode = &mut pre_head;
            while let Some(next_probe_ref) = (*probe).next.as_ref() {
                probe = next_probe_ref.as_ref();
                if let Some(next_node) = (*pre_rm_node).next.as_mut() {
                    pre_rm_node = next_node.as_mut();
                }
            }
            &mut *pre_rm_node
        };
        let mut rm_node = pre_rm_node.next.take();
        pre_rm_node.next = rm_node.as_mut().and_then(|rm_node| rm_node.next.take());
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
            Solution::remove_nth_from_end(linkedlist![1, 2, 3, 4, 5], 2),
            linkedlist![1, 2, 3, 5]
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::remove_nth_from_end(linkedlist![1], 1), None);
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::remove_nth_from_end(linkedlist![1, 2], 1),
            linkedlist![1]
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::remove_nth_from_end(None, 5), None);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::remove_nth_from_end(linkedlist![9, -100, 6, 0, 6, 6, 9], 3));
    }
}
