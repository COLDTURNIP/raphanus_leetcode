/*
Problem 11. Container With Most Water
=====================================

https://leetcode.com/problems/container-with-most-water/

Given n non-negative integers a1, a2, ..., an , where each represents a point at coordinate
(i, ai). n vertical lines are drawn such that the two endpoints of line i is at (i, ai) and (i, 0).
Find two lines, which together with x-axis forms a container, such that the container contains the
most water.

    https://s3-lc-upload.s3.amazonaws.com/uploads/2018/07/17/question_11.jpg

The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area
of water (blue section) the container can contain is 49.

Note: You may not slant the container and n is at least 2.

Example:

    Input: [1,8,6,2,5,4,8,3,7]
    Output: 49
*/

use std::cmp::Ordering::{Equal, Greater, Less};

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        // Brute-force:
        //
        // let len = height.len();
        // return (0..len - 1)
        //     .flat_map(|i| (i..len).map(move |j| (i, j)))
        //     .map(|(i, j)| (j - i) as i32 * height[i].min(height[j]))
        //     .max()
        //     .unwrap_or(0);

        if height.len() < 2 {
            return 0;
        }
        let len = height.len();
        let (mut li, mut lh) = (0, height[0]);
        let (mut ri, mut rh) = (len - 1, height[len - 1]);
        let mut ans = 0;
        while li < ri {
            ans = ans.max((ri - li) as i32 * lh.min(rh));
            match lh.cmp(&rh) {
                Less => match (li + 1..ri).find(|&i| height[i] > lh) {
                    Some(i) => {
                        li = i;
                        lh = height[i];
                    }
                    None => break,
                },
                Greater => match (li + 1..ri).rev().find(|&i| height[i] > rh) {
                    Some(i) => {
                        ri = i;
                        rh = height[i];
                    }
                    None => break,
                },
                Equal => {
                    let left = (li + 1..ri).find(|&i| height[i] > lh);
                    let right = (li + 1..ri).rev().find(|&i| height[i] > rh);
                    match (left, right) {
                        (Some(nli), Some(nri)) if nli < nri => {
                            let nlh = height[nli];
                            let nrh = height[nri];
                            if nlh > nrh {
                                li = nli;
                                lh = nlh;
                            } else {
                                ri = nri;
                                rh = nrh;
                            }
                        }
                        _ => break,
                    }
                }
            }
        }
        ans
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    #[test]
    fn test_87() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn test_78() {
        assert_eq!(Solution::max_area(vec![1, 7, 6, 2, 5, 4, 8, 3, 8]), 49);
    }

    #[test]
    fn test_10_9() {
        assert_eq!(Solution::max_area(vec![2, 3, 10, 5, 7, 8, 9]), 36);
    }

    #[test]
    fn test_some() {
        assert_eq!(
            Solution::max_area(vec![6, 4, 3, 1, 4, 6, 99, 62, 1, 2, 6]),
            62
        );
    }

    #[test]
    fn test_long() {
        assert_eq!(
            Solution::max_area(vec![
                76, 155, 15, 188, 180, 154, 84, 34, 187, 142, 22, 5, 27, 183, 111, 128, 50, 58, 2,
                112, 179, 2, 100, 111, 115, 76, 134, 120, 118, 103, 31, 146, 58, 198, 134, 38, 104,
                170, 25, 92, 112, 199, 49, 140, 135, 160, 20, 185, 171, 23, 98, 150, 177, 198, 61,
                92, 26, 147, 164, 144, 51, 196, 42, 109, 194, 177, 100, 99, 99, 125, 143, 12, 76,
                192, 152, 11, 152, 124, 197, 123, 147, 95, 73, 124, 45, 86, 168, 24, 34, 133, 120,
                85, 81, 163, 146, 75, 92, 198, 126, 191
            ]),
            18048
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]))
    }
}
