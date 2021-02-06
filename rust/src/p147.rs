/*
Problem 147. Insertion Sort List
================================

https://leetcode.com/problems/insertion-sort-list/

Sort a linked list using insertion sort.

https://upload.wikimedia.org/wikipedia/commons/0/0f/Insertion-sort-example-300px.gif

    - A graphical example of insertion sort. The partial sorted list (black) initially contains
      only the first element in the list.
    - With each iteration one element (red) is removed from the input data and inserted in-place
      into the sorted list

Algorithm of Insertion Sort:

    - Insertion sort iterates, consuming one input element each repetition, and growing a sorted
      output list.
    - At each iteration, insertion sort removes one element from the input data, finds the location
      it belongs within the sorted list, and inserts it there.
    - It repeats until no input elements remain.

Example 1:

    Input: 4->2->1->3
    Output: 1->2->3->4

Example 2:

    Input: -1->5->3->4->0
    Output: -1->0->3->4->5
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
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // This implementation only faster than 57.14% in LeetCode since it strictly follows the
        // insertion sort algorithm.
        let mut unsorted = head;
        let mut sorted = ListNode {
            next: None,
            val: std::i32::MIN,
        };

        while let Some(mut new_node) = unsorted.take() {
            unsorted = new_node.next.take();
            let val = new_node.val;
            let mut node_ref = &mut sorted;
            loop {
                if node_ref
                    .next
                    .as_ref()
                    .filter(|next_ref| next_ref.val <= val)
                    .is_some()
                {
                    node_ref = node_ref.next.as_mut().unwrap();
                } else {
                    new_node.next = node_ref.next.take();
                    node_ref.next = Some(new_node);
                    break;
                }
            }
        }
        sorted.next.take()
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
            Solution::insertion_sort_list(linkedlist![4, 2, 1, 3]),
            linkedlist![1, 2, 3, 4]
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::insertion_sort_list(linkedlist![-1, 5, 3, 4, 0]),
            linkedlist![-1, 0, 3, 4, 5]
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::insertion_sort_list(None), None);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::insertion_sort_list(linkedlist![9, -100, 6, 0, 6, 6, 9]));
    }
}
