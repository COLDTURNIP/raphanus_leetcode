/*
Problem 23. Merge k Sorted Lists
================================

https://leetcode.com/problems/merge-k-sorted-lists/

You are given an array of k linked-lists lists, each linked-list is sorted in ascending order.

Merge all the linked-lists into one sorted linked-list and return it.

Example 1:

    Input: lists = [[1,4,5],[1,3,4],[2,6]]
    Output: [1,1,2,3,4,4,5,6]
    Explanation: The linked-lists are:
    [
      1->4->5,
      1->3->4,
      2->6
    ]
    merging them into one sorted list:
    1->1->2->3->4->4->5->6

Example 2:

    Input: lists = []
    Output: []

Example 3:

    Input: lists = [[]]
    Output: []

Constraints:

    k == lists.length
    0 <= k <= 10^4
    0 <= lists[i].length <= 500
    -10^4 <= lists[i][j] <= 10^4
    lists[i] is sorted in ascending order.
    The sum of lists[i].length won't exceed 10^4.
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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists = lists;
        while lists.len() > 1 {
            // perform merge sort for each 2 costs only log(n) each merging iteration
            let merged_num = (lists.len() + 1) / 2;
            for i in 0..merged_num {
                let list1 = lists[i * 2].take();
                let list2 = if i * 2 + 1 < lists.len() {
                    lists[i * 2 + 1].take()
                } else {
                    None
                };
                lists[i] = Self::merge_lists(list1, list2);
            }
            lists.truncate(merged_num);
        }
        if lists.is_empty() {
            None
        } else {
            lists[0].take()
        }
    }

    fn merge_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut ret = Some(ListNode::new(0));
        let mut tail = ret.as_mut().unwrap();
        let mut lists = (list1, list2);
        loop {
            lists = match (&lists.0, &lists.1) {
                (&None, &None) => break,
                (&Some(_), &None) => lists,
                (&None, &Some(_)) => (lists.1, lists.0),
                (&Some(ref n1), &Some(ref n2)) => {
                    if n1.val <= n2.val {
                        lists
                    } else {
                        (lists.1, lists.0)
                    }
                }
            };
            if let Some(mut next_node) = lists.0 {
                tail.next = Some(Box::new(ListNode::new(next_node.val)));
                tail = tail.next.as_mut().unwrap();
                lists.0 = next_node.next.take();
            } else {
                unreachable!();
            }
        }
        ret.unwrap().next
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
    fn test_merge_sort() {
        assert_eq!(
            Solution::merge_k_lists(vec![
                linkedlist![1, 4, 5],
                linkedlist![1, 3, 4],
                linkedlist![2, 6]
            ],),
            linkedlist![1, 1, 2, 3, 4, 4, 5, 6],
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::merge_k_lists(vec![None]), None);
        assert_eq!(Solution::merge_k_lists(Vec::new()), None);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::merge_k_lists(vec![
                linkedlist![1, 4, 5, 9, 9, 9, 9, 9, 9, 9],
                linkedlist![2, 6],
                linkedlist![1, 3, 4, 9, 10, 11, 11, 19],
            ])
        });
    }
}
