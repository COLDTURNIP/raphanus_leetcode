/*
Problem 98. Validate Binary Search Tree
=======================================

https://leetcode.com/problems/validate-binary-search-tree/

Given a binary tree, determine if it is a valid binary search tree (BST).

Assume a BST is defined as follows:

The left subtree of a node contains only nodes with keys less than the node's key.
The right subtree of a node contains only nodes with keys greater than the node's key.
Both the left and right subtrees must also be binary search trees.


Example 1:

        2
       / \
      1   3

    Input: [2,1,3]
    Output: true

Example 2:

        5
       / \
      1   4
         / \
        3   6

    Input: [5,1,4,null,null,3,6]
    Output: false
    Explanation: The root node's value is 5 but its right child's value is 4.
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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::valid_bst(&root, None, None)
    }

    fn valid_bst(root: &Option<Rc<RefCell<TreeNode>>>, lb: Option<i32>, rb: Option<i32>) -> bool {
        if let Some(ref rcnode) = root {
            let node = rcnode.borrow();
            if let Some(lb) = lb {
                if node.val <= lb {
                    return false;
                }
            }
            if let Some(rb) = rb {
                if node.val >= rb {
                    return false;
                }
            }
            Self::valid_bst(&node.left, lb, Some(node.val))
                && Self::valid_bst(&node.right, Some(node.val), rb)
        } else {
            true
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
    fn test_valid() {
        assert!(Solution::is_valid_bst(btree![2, 1, 3]));
    }

    #[test]
    fn test_invalid() {
        assert!(!Solution::is_valid_bst(btree![5, 1, 4, null, null, 3, 6]));
    }

    #[test]
    fn test_empty() {
        assert!(Solution::is_valid_bst(None));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::is_valid_bst(btree![1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4]));
    }
}
