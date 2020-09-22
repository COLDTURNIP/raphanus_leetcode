/*
Problem 94. Binary Tree Inorder Traversal
=========================================

https://leetcode.com/problems/binary-tree-inorder-traversal/

Given the root of a binary tree, return the inorder traversal of its nodes' values.

Follow up: Recursive solution is trivial, could you do it iteratively?

Example 1:

    Input: root = [1,null,2,3]
    Output: [1,3,2]

Example 2:

    Input: root = []
    Output: []

Example 3:

    Input: root = [1]
    Output: [1]

Example 4:

    Input: root = [1,2]
    Output: [2,1]

Example 5:

    Input: root = [1,null,2]
    Output: [1,2]

Constraints:

    The number of nodes in the tree is in the range [0, 100].
    -100 <= Node.val <= 100
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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut buf = Vec::new();
        buf.push((root, true));
        while let Some(item) = buf.pop() {
            match item {
                (None, _) => (),
                (Some(node_ref), false) => ans.push(node_ref.borrow().val),
                (Some(node_ref), true) => {
                    let left = node_ref.borrow().left.clone();
                    let right = node_ref.borrow().right.clone();
                    buf.push((right, true));
                    buf.push((Some(node_ref), false));
                    buf.push((left, true));
                }
            }
        }
        ans
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
    fn test_132() {
        assert_eq!(
            Solution::inorder_traversal(btree![1, null, 2, 3]),
            vec![1, 3, 2]
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::inorder_traversal(btree![1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4]));
    }
}
