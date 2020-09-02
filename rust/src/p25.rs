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
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k < 2 {
            return head;
        }
        let k = k as usize;
        let mut ret_head = Box::new(ListNode::new(0));
        let mut head = head;
        let mut prev_tail = &mut ret_head;
        let mut cur_buf = Vec::with_capacity(k);
        loop {
            match head {
                Some(cur_node) => {
                    cur_buf.push(cur_node.val);
                    head = cur_node.next;
                }
                None => break,
            }
            if cur_buf.len() == k {
                for i in 1..=k {
                    prev_tail.next = Some(Box::new(ListNode::new(cur_buf[k - i])));
                    prev_tail = prev_tail.next.as_mut().unwrap();
                }
                cur_buf.truncate(0);
            }
        }
        for n in cur_buf.into_iter() {
            prev_tail.next = Some(Box::new(ListNode::new(n)));
            prev_tail = prev_tail.next.as_mut().unwrap();
        }
        ret_head.next
    }
}

use crate::ListNode;
pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;
    use crate::linkedlist;

    #[test]
    fn test_reverse_k_group() {
        assert_eq!(
            Solution::reverse_k_group(linkedlist![1, 2, 3, 4], 2),
            linkedlist![2, 1, 4, 3]
        );
        assert_eq!(
            Solution::reverse_k_group(linkedlist![1, 2, 3, 4, 5], 2),
            linkedlist![2, 1, 4, 3, 5]
        );
        assert_eq!(
            Solution::reverse_k_group(linkedlist![1, 2, 3, 4, 5], 3),
            linkedlist![3, 2, 1, 4, 5]
        );
        assert_eq!(Solution::reverse_k_group(linkedlist![], 4), linkedlist![]);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::reverse_k_group(linkedlist![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4));
    }
}
