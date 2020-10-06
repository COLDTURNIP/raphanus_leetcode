/*
Problem 129. Sum Root to Leaf Numbers
=====================================

https://leetcode.com/problems/sum-root-to-leaf-numbers/

Given a binary tree containing digits from 0-9 only, each root-to-leaf path could represent a
number.

An example is the root-to-leaf path 1->2->3 which represents the number 123.

Find the total sum of all root-to-leaf numbers.

Note: A leaf is a node with no children.

Example:

    Input: [1,2,3]
        1
       / \
      2   3

    Output: 25
    Explanation:
        The root-to-leaf path 1->2 represents the number 12.
        The root-to-leaf path 1->3 represents the number 13.
        Therefore, sum = 12 + 13 = 25.

Example 2:

    Input: [4,9,0,5,1]
        4
       / \
      9   0
     / \
    5   1

    Output: 1026
    Explanation:
        The root-to-leaf path 4->9->5 represents the number 495.
        The root-to-leaf path 4->9->1 represents the number 491.
        The root-to-leaf path 4->0 represents the number 40.
        Therefore, sum = 495 + 491 + 40 = 1026.
*/

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::sum_tree(&root, 0).unwrap_or(0)
    }

    fn sum_tree(root: &Option<Rc<RefCell<TreeNode>>>, path_sum: i32) -> Option<i32> {
        root.as_ref().map(|node_rc| {
            let node = node_rc.borrow();
            let path_sum = path_sum * 10 + node.val;
            match (
                Self::sum_tree(&node.left, path_sum),
                Self::sum_tree(&node.right, path_sum),
            ) {
                (Some(left), Some(right)) => left + right,
                (Some(val), None) | (None, Some(val)) => val,
                (None, None) => path_sum,
            }
        })
    }
}

pub struct Solution;
use leetcode_prelude::TreeNode;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;
    use leetcode_prelude::btree;

    #[test]
    fn test_123() {
        assert_eq!(Solution::sum_numbers(btree![1, 2, 3]), 25);
    }

    #[test]
    fn test_49051() {
        assert_eq!(Solution::sum_numbers(btree![4, 9, 0, 5, 1]), 1026);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::sum_numbers(btree![4, 9, 0, 5, 1]);
        });
    }
}
