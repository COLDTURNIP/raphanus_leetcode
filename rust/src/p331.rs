/*
Problem 331. Verify Preorder Serialization of a Binary Tree
===========================================================

https://leetcode.com/problems/verify-preorder-serialization-of-a-binary-tree/

One way to serialize a binary tree is to use pre-order traversal. When we encounter a non-null
node, we record the node's value. If it is a null node, we record using a sentinel value such as #.

         _9_
        /   \
       3     2
      / \   / \
     4   1  #  6
    / \ / \   / \
    # # # #   # #

For example, the above binary tree can be serialized to the string "9,3,4,#,#,1,#,#,2,#,6,#,#",
where # represents a null node.

Given a string of comma separated values, verify whether it is a correct preorder traversal
serialization of a binary tree. Find an algorithm without reconstructing the tree.

Each comma separated value in the string must be either an integer or a character '#' representing
null pointer.

You may assume that the input format is always valid, for example it could never contain two
consecutive commas such as "1,,3".

Example 1:

    Input: "9,3,4,#,#,1,#,#,2,#,6,#,#"
    Output: true

Example 2:

    Input: "1,#"
    Output: false

Example 3:

    Input: "9,#,#,1"
    Output: false
*/

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut state = Vec::new();
        let mut node_iter = preorder.split(',');
        let mut cur_node_leaf_num = if let Some(root) = node_iter.next() {
            if root == "#" {
                0
            } else {
                2
            }
        } else {
            return false;
        };
        loop {
            if cur_node_leaf_num > 0 {
                state.push(cur_node_leaf_num - 1);
                cur_node_leaf_num = if let Some(node) = node_iter.next() {
                    if node == "#" {
                        0
                    } else {
                        2
                    }
                } else {
                    return false;
                };
            } else if let Some(num) = state.pop() {
                cur_node_leaf_num = num;
            } else {
                return node_iter.next().is_none();
            }
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert!(Solution::is_valid_serialization(String::from(
            "9,3,4,#,#,1,#,#,2,#,6,#,#"
        )));
    }

    #[test]
    fn test_case2() {
        assert!(!Solution::is_valid_serialization(String::from("1,#")));
    }

    #[test]
    fn test_case3() {
        assert!(!Solution::is_valid_serialization(String::from("9,#,#,1")));
    }

    #[test]
    fn test_case4() {
        assert!(Solution::is_valid_serialization(String::from("#")));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::is_valid_serialization(String::from("9,3,4,#,#,1,#,#,2,#,6,#,#")));
    }
}
