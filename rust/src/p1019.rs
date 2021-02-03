/*
Problem 1019. Next Greater Node In Linked List
==============================================

We are given a linked list with head as the first node.  Let's number the nodes in the list:
node_1, node_2, node_3, ... etc.

Each node may have a next larger value: for node_i, next_larger(node_i) is the node_j.val such that
j > i, node_j.val > node_i.val, and j is the smallest possible choice.  If such a j does not exist,
the next larger value is 0.

Return an array of integers answer, where answer[i] = next_larger(node_{i+1}).

Note that in the example inputs (not outputs) below, arrays such as [2,1,5] represent the
serialization of a linked list with a head node value of 2, second node value of 1, and third node
value of 5.

Example 1:

    Input: [2,1,5]
    Output: [5,5,0]

Example 2:

    Input: [2,7,4,3,5]
    Output: [7,0,5,5,0]

Example 3:

    Input: [1,7,5,1,9,2,5,1]
    Output: [7,9,9,9,0,5,0,0]

Note:

    - 1 <= node.val <= 10^9 for each node in the linked list.
    - The given list has length in the range [0, 10000].
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
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut buf = Vec::new();
        let mut head = head;
        for i in 0.. {
            let n = if let Some(head_ref) = head {
                head = head_ref.next;
                head_ref.val
            } else {
                break;
            };
            let mut buf_len = buf.len();
            for tail_idx in (0..buf_len).rev() {
                let (prev_n, prev_i) = buf[tail_idx];
                if prev_n >= n {
                    break;
                } else {
                    buf_len = tail_idx;
                    ans[prev_i] = n;
                }
            }
            buf.truncate(buf_len);
            buf.push((n, i));
            ans.push(0);
        }
        ans
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
            Solution::next_larger_nodes(linkedlist![2, 1, 5]),
            vec![5, 5, 0]
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::next_larger_nodes(linkedlist![2, 7, 4, 3, 5]),
            vec![7, 0, 5, 5, 0]
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::next_larger_nodes(linkedlist![1, 7, 5, 1, 9, 2, 5, 1]),
            vec![7, 9, 9, 9, 0, 5, 0, 0]
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::next_larger_nodes(None), Vec::new());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::next_larger_nodes(linkedlist![1, 7, 5, 1, 9, 2, 5, 1]));
    }
}
