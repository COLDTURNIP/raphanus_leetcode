/*
2. Add Two Numbers
==================

https://leetcode.com/problems/add-two-numbers/

You are given two non-empty linked lists representing two non-negative integers. The digits are
stored in reverse order and each of their nodes contain a single digit. Add the two numbers and
return it as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.

Example:

    Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
    Output: 7 -> 0 -> 8
    Explanation: 342 + 465 = 807.
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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut carry = 0;
        let mut ans = Some(Box::new(ListNode { next: None, val: 0 }));
        let mut prev = &mut ans;
        while l1.is_some() || l2.is_some() || carry > 0 {
            let (cur_digit, next_carry) = match (l1.take(), l2.take(), carry) {
                (Some(n1), Some(n2), c) => {
                    l1 = (*n1).next;
                    l2 = (*n2).next;
                    ((n1.val + n2.val + c) % 10, (n1.val + n2.val + c) / 10)
                }
                (Some(n), None, c) | (None, Some(n), c) => {
                    l1 = (*n).next;
                    l2 = None;
                    ((n.val + c) % 10, (n.val + c) / 10)
                }
                (None, None, c) => (c, 0),
            };
            if cur_digit > 0 || next_carry > 0 || l1.is_some() || l2.is_some() {
                carry = next_carry;
                prev.as_mut().into_iter().for_each(|boxed_node| {
                    boxed_node.next = Some(Box::new(ListNode {
                        val: cur_digit,
                        next: None,
                    }))
                });
                prev = &mut prev.as_mut().unwrap().next;
            } else {
                break;
            }
        }
        ans.unwrap()
            .next
            .or_else(|| Some(Box::new(ListNode { val: 0, next: None })))
    }
}

use crate::ListNode;
pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;
    use leetcode_prelude::linkedlist;

    #[test]
    fn test_sum() {
        assert_eq!(
            Solution::add_two_numbers(linkedlist![2, 4, 3], linkedlist![5, 6, 4]),
            linkedlist![7, 0, 8]
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            Solution::add_two_numbers(linkedlist![0], linkedlist![0]),
            linkedlist![0]
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::add_two_numbers(linkedlist![2, 4, 3], linkedlist![5, 6, 4]))
    }
}
