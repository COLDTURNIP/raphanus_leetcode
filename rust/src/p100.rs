/*
Problem 100. Same Tree
======================

https://leetcode.com/problems/same-tree/

Given two binary trees, write a function to check if they are the same or not.

Two binary trees are considered the same if they are structurally identical and the nodes have the same value.

Example 1:

    Input:     1         1
              / \       / \
             2   3     2   3

            [1,2,3],   [1,2,3]

    Output: true

Example 2:

    Input:     1         1
              /           \
             2             2

            [1,2],     [1,null,2]

    Output: false

Example 3:

    Input:     1         1
              / \       / \
             2   1     1   2

            [1,2,1],   [1,1,2]

    Output: false
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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::is_same(&p, &q)
    }

    fn is_same(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p), Some(q)) => {
                let rp = p.borrow();
                let rq = q.borrow();
                rp.val == rq.val
                    && Self::is_same(&rp.left, &rq.left)
                    && Self::is_same(&rp.right, &rq.right)
            }
            _ => false,
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
    fn test_same() {
        assert!(Solution::is_same_tree(btree![1, 2, 3], btree![1, 2, 3]));
    }

    #[test]
    fn test_same_bias() {
        assert!(!Solution::is_same_tree(btree![1, 2], btree![1, null, 2]));
    }

    #[test]
    fn test_different_switch() {
        assert!(!Solution::is_same_tree(btree![1, 2, 1], btree![1, 1, 2]));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::is_same_tree(
                btree![1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4],
                btree![1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4],
            )
        });
    }
}
