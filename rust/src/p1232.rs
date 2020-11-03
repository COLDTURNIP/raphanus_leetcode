/*
Problem 1232. Check If It Is a Straight Line
============================================

https://leetcode.com/problems/check-if-it-is-a-straight-line/

You are given an array coordinates, coordinates[i] = [x, y], where [x, y] represents the coordinate
of a point. Check if these points make a straight line in the XY plane.

Example 1:

    https://assets.leetcode.com/uploads/2019/10/15/untitled-diagram-2.jpg

    Input: coordinates = [[1,2],[2,3],[3,4],[4,5],[5,6],[6,7]]
    Output: true

Example 2:

    https://assets.leetcode.com/uploads/2019/10/09/untitled-diagram-1.jpg

    Input: coordinates = [[1,1],[2,2],[3,4],[4,5],[5,6],[7,7]]
    Output: false

Constraints:

    - 2 <= coordinates.length <= 1000
    - coordinates[i].length == 2
    - -10^4 <= coordinates[i][0], coordinates[i][1] <= 10^4
    - coordinates contains no duplicate point.
*/

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let mut node_iter = coordinates.into_iter();
        let node1 = if let Some(first) = node_iter.next() {
            (first[0], first[1])
        } else {
            return true;
        };
        let node2 =
            if let Some(node) = node_iter.find(|node| node[0] != node1.0 || node[1] != node1.1) {
                (node[0], node[1])
            } else {
                return true;
            };
        let x_diff = node2.0 - node1.0;
        let y_diff = node2.1 - node1.1;
        node_iter.all(|node| y_diff * (node[0] - node1.0) == x_diff * (node[1] - node1.1))
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_straight() {
        assert!(Solution::check_straight_line(vec![
            vec![4, 5],
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![5, 6],
            vec![6, 7]
        ]));
        assert!(Solution::check_straight_line(vec![
            vec![0, 0],
            vec![1, 0],
            vec![-1, 0]
        ]));
        assert!(Solution::check_straight_line(vec![
            vec![0, 0],
            vec![0, 1],
            vec![0, -1]
        ]));
    }

    #[test]
    fn test_non_straight() {
        assert!(!Solution::check_straight_line(vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![7, 7]
        ]));
    }

    #[test]
    fn test_empty() {
        assert!(Solution::check_straight_line(Vec::new()));
    }

    #[test]
    fn test_single_node() {
        assert!(Solution::check_straight_line(vec![vec![1, 1]]));
    }

    #[test]
    fn test_two_nodes() {
        assert!(Solution::check_straight_line(vec![vec![1, 1], vec![4, 5]]));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::check_straight_line(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![4, 5],
                vec![5, 6],
                vec![6, 7],
            ])
        });
    }
}
