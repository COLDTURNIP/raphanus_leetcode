/*
Problem 83. Remove Duplicates from Sorted List
==============================================

https://leetcode.com/problems/remove-duplicates-from-sorted-list/

Given a sorted linked list, delete all duplicates such that each element appear only once.

Example 1:

    Input: 1->1->2
    Output: 1->2

Example 2:

    Input: 1->1->2->3->3
    Output: 1->2->3
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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut head = head;
        let mut cur = head.as_mut().unwrap().as_mut();
        loop {
            match cur.next.as_ref() {
                Some(next) if next.val == cur.val => {
                    let next = cur.next.as_mut().unwrap().next.take();
                    cur.next = next;
                }
                Some(_) => {
                    cur = cur.next.as_mut().unwrap().as_mut();
                }
                None => break,
            }
        }
        head
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
    fn test_112() {
        assert_eq!(
            Solution::delete_duplicates(linkedlist![1, 1, 2]),
            linkedlist![1, 2]
        );
    }

    #[test]
    fn test_11233() {
        assert_eq!(
            Solution::delete_duplicates(linkedlist![1, 1, 2, 3, 3]),
            linkedlist![1, 2, 3]
        );
    }

    #[test]
    fn test_zeros() {
        assert_eq!(
            Solution::delete_duplicates(linkedlist![0, 0, 0, 0, 0]),
            linkedlist![0]
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::delete_duplicates(None), None);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::delete_duplicates(linkedlist![1, 1, 2, 3, 3, 4, 4, 4, 4, 5]));
    }
}
