/*
Problem 148. Sort List
======================

https://leetcode.com/problems/sort-list/

Given the head of a linked list, return the list after sorting it in ascending order.

Follow up: Can you sort the linked list in O(n logn) time and O(1) memory (i.e. constant space)?

Example 1:

    Input: head = [4,2,1,3]
    Output: [1,2,3,4]

Example 2:

    Input: head = [-1,5,3,4,0]
    Output: [-1,0,3,4,5]

Example 3:

    Input: head = []
    Output: []

Constraints:

    - The number of nodes in the list is in the range [0, 5 * 10^4].
    - -10^5 <= Node.val <= 10^5
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
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (head1, head2) = Self::split_list(head);
        match (head1.is_none(), head2.is_none()) {
            (_, true) => head1,
            (true, _) => head2,
            _ => {
                let head1 = Self::sort_list(head1);
                let head2 = Self::sort_list(head2);
                Self::merge_list(head1, head2)
            }
        }
    }

    fn split_list(head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut pre_head = ListNode { next: head, val: 0 };
        let head2 = unsafe { // two cursors for performance
            let mut tail1: *mut ListNode = &mut pre_head;
            let mut tail2: *mut ListNode = &mut pre_head;
            loop {
                if let Some(tail_ref) = (*tail2).next.as_mut().and_then(|tr| tr.next.as_mut()) {
                    tail2 = tail_ref.as_mut();
                } else {
                    break;
                }
                if let Some(tail_ref) = (*tail1).next.as_mut() {
                    tail1 = tail_ref.as_mut();
                }
            }
            (*tail1).next.take()
        };
        (pre_head.next, head2)
    }

    fn merge_list(
        mut head1: Option<Box<ListNode>>,
        mut head2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut sorted = ListNode { next: None, val: 0 };
        let mut tail = &mut sorted;
        loop {
            let picked = match (head1.as_mut(), head2.as_mut()) {
                (None, None) => break,
                (Some(n1), Some(n2)) => {
                    if n1.val <= n2.val {
                        &mut head1
                    } else {
                        &mut head2
                    }
                }
                (Some(_), None) => &mut head1,
                (None, Some(_)) => &mut head2,
            };
            tail.next = picked.take();
            tail = tail.next.as_mut().unwrap();
            *picked = tail.next.take();
        }
        sorted.next
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
            Solution::sort_list(linkedlist![4, 2, 1, 3]),
            linkedlist![1, 2, 3, 4]
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::sort_list(linkedlist![-1, 5, 3, 4, 0]),
            linkedlist![-1, 0, 3, 4, 5]
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::sort_list(None), None);
    }

    #[test]
    fn test_split() {
        assert_eq!(
            Solution::split_list(linkedlist![1, 2, 3]),
            (linkedlist![1], linkedlist![2, 3])
        );
        assert_eq!(Solution::split_list(None), (None, None));
        assert_eq!(
            Solution::split_list(linkedlist![1, 2]),
            (linkedlist![1], linkedlist![2])
        );
        assert_eq!(Solution::split_list(linkedlist![1]), (None, linkedlist![1]));
    }

    #[test]
    fn test_merge() {
        assert_eq!(
            Solution::merge_list(linkedlist![1, 3], linkedlist![2]),
            linkedlist![1, 2, 3]
        );
        assert_eq!(Solution::merge_list(None, None), None);
        assert_eq!(Solution::merge_list(linkedlist![1], None), linkedlist![1]);
        assert_eq!(Solution::merge_list(None, linkedlist![1]), linkedlist![1]);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::sort_list(linkedlist![9, -100, 6, 0, 6, 6, 9]));
    }
}
