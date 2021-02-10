/*
Problem 82. Remove Duplicates from Sorted List II
=================================================

https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/

Given the head of a sorted linked list, delete all nodes that have duplicate numbers, leaving only
distinct numbers from the original list. Return the linked list sorted as well.

Example 1:

    https://assets.leetcode.com/uploads/2021/01/04/linkedlist1.jpg

    Input: head = [1,2,3,3,4,4,5]
    Output: [1,2,5]

Example 2:

    https://assets.leetcode.com/uploads/2021/01/04/linkedlist2.jpg

    Input: head = [1,1,1,2,3]
    Output: [2,3]

Constraints:

    - The number of nodes in the list is in the range [0, 300].
    - -100 <= Node.val <= 100
    - The list is guaranteed to be sorted in ascending order.
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
        let mut pre_head = Box::new(ListNode { next: None, val: 0 });
        let mut tail = &mut pre_head;
        let mut unprocess = head;
        let mut last_node = None;
        let mut insert_last = false;
        loop {
            let mut cur_node = unprocess;
            unprocess = if let Some(ref mut cur_node_ref) = cur_node {
                cur_node_ref.next.take()
            } else {
                break;
            };
            if cur_node != last_node {
                if insert_last {
                    tail.next = last_node;
                    tail = tail.next.as_mut().unwrap();
                }
                last_node = cur_node;
                insert_last = true;
            } else {
                insert_last = false;
            }
        }
        if insert_last {
            tail.next = last_node;
        }
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
            Solution::delete_duplicates(linkedlist![1, 2, 3, 3, 4, 4, 5]),
            linkedlist![1, 2, 5]
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::delete_duplicates(linkedlist![1, 1, 1, 2, 3]),
            linkedlist![2, 3]
        );
    }
    #[test]
    fn test_case3() {
        assert_eq!(Solution::delete_duplicates(linkedlist![1, 1, 1]), None);
    }

    #[test]
    fn test_case4() {
        assert_eq!(
            Solution::delete_duplicates(linkedlist![99]),
            linkedlist![99]
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::delete_duplicates(None), None);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::delete_duplicates(linkedlist![1, 2, 3, 3, 4, 4, 5]));
    }
}
