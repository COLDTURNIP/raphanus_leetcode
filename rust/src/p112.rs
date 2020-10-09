/*
Problem 112. Path Sum
=====================

https://leetcode.com/problems/path-sum/

Given a binary tree and a sum, determine if the tree has a root-to-leaf path such that adding up
all the values along the path equals the given sum.

Note: A leaf is a node with no children.

Example:

Given the below binary tree and sum = 22,

      5
     / \
    4   8
   /   / \
  11  13  4
 /  \      \
7    2      1
return true, as there exist a root-to-leaf path 5->4->11->2 which sum is 22.
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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        root.is_some() && Self::has_path_sum_internal(&root, sum)
    }

    fn has_path_sum_internal(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        match root {
            None => unreachable!(),
            Some(ref node_ref) => {
                let node = node_ref.borrow();
                let cur_sum = sum - node.val;
                match (&node.left, &node.right) {
                    (None, None) => cur_sum == 0,
                    (next, None) | (None, next) => Self::has_path_sum_internal(&next, cur_sum),
                    _ => {
                        Self::has_path_sum_internal(&node.left, cur_sum)
                            || Self::has_path_sum_internal(&node.right, cur_sum)
                    }
                }
            }
        }
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
    fn test_possible() {
        assert!(Solution::has_path_sum(
            btree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, null, 1],
            22
        ));
    }

    #[test]
    fn test_impossible() {
        assert!(!Solution::has_path_sum(
            btree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, null, 1],
            1000
        ));
    }

    #[test]
    fn test_possible_non_leaf() {
        assert!(!Solution::has_path_sum(btree![1, 2], 1));
    }

    #[test]
    fn test_empty() {
        assert!(!Solution::has_path_sum(None, 0));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::has_path_sum(
                btree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, null, 1],
                22,
            )
        });
    }
}
