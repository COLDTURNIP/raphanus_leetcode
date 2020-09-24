/*
Problem 118. Pascal's Triangle
==============================

https://leetcode.com/problems/pascals-triangle/

Given a non-negative integer numRows, generate the first numRows of Pascal's triangle.

In Pascal's triangle, each number is the sum of the two numbers directly above it.

Example:

    Input: 5
    Output:
    [
         [1],
        [1,1],
       [1,2,1],
      [1,3,3,1],
     [1,4,6,4,1]
    ]
*/

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        if num_rows < 1 {
            return Vec::new();
        }
        let len = num_rows as usize;
        let mut ans = Vec::with_capacity(len);
        ans.push(vec![1]);
        for level in 1..len {
            let mut entry = Vec::with_capacity(level + 1);
            let prev = &ans[level - 1];
            for i in 0..prev.len() {
                entry.push(i.checked_sub(1).and_then(|i| prev.get(i)).unwrap_or(&0) + prev[i]);
            }
            entry.push(1);
            ans.push(entry);
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
    fn test_5() {
        assert_eq!(
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(Solution::generate(0), Vec::<Vec<i32>>::new());
        assert_eq!(Solution::generate(-1), Vec::<Vec<i32>>::new());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::generate(10));
    }
}
