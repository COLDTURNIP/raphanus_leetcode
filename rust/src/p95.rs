/*
Problem 95. Unique Binary Search Trees II
=========================================

Given an integer n, generate all structurally unique BST's (binary search trees) that store values
1 ... n.

Example:

    Input: 3
    Output:
    [
      [1,null,3,2],
      [3,2,null,1],
      [3,1,null,null,2],
      [2,1,3],
      [1,null,2,null,3]
    ]
    Explanation:
    The above output corresponds to the 5 unique BST's shown below:

       1         3     3      2      1
        \       /     /      / \      \
         3     2     1      1   3      2
        /     /       \                 \
       2     1         2                 3
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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n <= 0 {
            Vec::new()
        } else {
            Self::gen_sub_tree(&(1..=n).collect::<Vec<i32>>())
        }
    }

    fn gen_sub_tree(vals: &[i32]) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if vals.is_empty() {
            return vec![None];
        } else if vals.len() == 1 {
            return vec![Some(Rc::new(RefCell::new(TreeNode::new(vals[0]))))];
        }
        let mut ans = Vec::new();
        for (idx, root) in vals.iter().enumerate() {
            let left_set = &vals[0..idx];
            let right_set = &vals[idx + 1..];
            let left_trees = Self::gen_sub_tree(left_set);
            let right_trees = Self::gen_sub_tree(right_set);
            for left_node in left_trees.iter() {
                for right_node in right_trees.iter() {
                    let root_node = TreeNode {
                        val: *root,
                        left: left_node.clone(),
                        right: right_node.clone(),
                    };
                    ans.push(Some(Rc::new(RefCell::new(root_node))));
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
    use leetcode_prelude::TreeNode;
    use std::cell::RefCell;
    use std::collections::BTreeSet;
    use std::rc::Rc;

    fn check(input: i32, expect_size: usize) {
        let output = Solution::generate_trees(input);
        let output_len = output.len();
        let output_set = output
            .into_iter()
            .collect::<BTreeSet<Option<Rc<RefCell<TreeNode>>>>>();
        assert_eq!(output_set.len(), output_len);
        assert_eq!(output_set.len(), expect_size);
    }

    #[test]
    fn test_3() {
        check(3, 5);
    }

    #[test]
    fn test_single_comb() {
        check(0, 0);
        check(1, 1);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::generate_trees(6));
    }
}
