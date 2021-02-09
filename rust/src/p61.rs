/*
Problem 61. Rotate List
=======================

https://leetcode.com/problems/rotate-list/

Given the head of a linked list, rotate the list to the right by k places.



Example 1:

    https://assets.leetcode.com/uploads/2020/11/13/rotate1.jpg
             1 -> 2 -> 3 -> 4 -> 5
    round 1: 5 -> 1 -> 2 -> 3 -> 4
    round 2: 4 -> 5 -> 1 -> 2 -> 3

    Input: head = [1,2,3,4,5], k = 2
    Output: [4,5,1,2,3]

Example 2:

    https://assets.leetcode.com/uploads/2020/11/13/roate2.jpg
             0 -> 1 -> 2
    round 1: 2 -> 0 -> 1
    round 2: 1 -> 2 -> 0
    round 3: 0 -> 1 -> 2
    round 4: 2 -> 0 -> 1

    Input: head = [0,1,2], k = 4
    Output: [2,0,1]

Constraints:

    - The number of nodes in the list is in the range [0, 500].
    - -100 <= Node.val <= 100
    - 0 <= k <= 2 * 10^9
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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k == 0 {
            return head;
        }
        unsafe {
            // two cursor for performance
            let mut pre_head = ListNode { next: head, val: 0 };
            let mut head_end: *mut ListNode = &mut pre_head;
            let mut len = 0;
            for i in 0..k {
                if let Some(node_ref) = (*head_end).next.as_mut() {
                    head_end = node_ref.as_mut();
                } else {
                    len = i;
                    break;
                }
            }
            if len > 0 {
                let k = k % len;
                if k == 0 {
                    return pre_head.next;
                }
                head_end = &mut pre_head;
                for _ in 0..k {
                    head_end = (*head_end).next.as_mut().unwrap().as_mut();
                }
            }
            let mut tail_end: *mut ListNode = &mut pre_head;
            while let Some(node_ref) = (*head_end).next.as_mut() {
                head_end = node_ref.as_mut();
                tail_end = (*tail_end).next.as_mut().unwrap().as_mut();
            }
            let head = (*tail_end).next.take();
            (*head_end).next = pre_head.next.take();
            head
        }
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
            Solution::rotate_right(linkedlist![1, 2, 3, 4, 5], 2),
            linkedlist![4, 5, 1, 2, 3]
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::rotate_right(linkedlist![0, 1, 2], 4),
            linkedlist![2, 0, 1]
        );
    }
    #[test]
    fn test_case3() {
        assert_eq!(Solution::rotate_right(linkedlist![1], 1), linkedlist![1]);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::rotate_right(None, 99), None);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::rotate_right(linkedlist![9, -100, 6, 0, 6, 6, 9], 100));
    }
}
