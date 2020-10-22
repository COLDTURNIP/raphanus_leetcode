/*
Problem 1470. Shuffle the Array
===============================

https://leetcode.com/problems/shuffle-the-array/

Given the array nums consisting of 2n elements in the form
[x1,x2,...,xn,y1,y2,...,yn].

Return the array in the form [x1,y1,x2,y2,...,xn,yn].

Example 1:

    Input: nums = [2,5,1,3,4,7], n = 3
    Output: [2,3,5,4,1,7]
    Explanation: Since x1=2, x2=5, x3=1, y1=3, y2=4, y3=7 then the answer is [2,3,5,4,1,7].

Example 2:

    Input: nums = [1,2,3,4,4,3,2,1], n = 4
    Output: [1,4,2,3,3,2,4,1]

Example 3:

    Input: nums = [1,1,2,2], n = 2
    Output: [1,2,1,2]

Constraints:

    - 1 <= n <= 500
    - nums.length == 2n
    - 1 <= nums[i] <= 10^3
*/

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        nums.iter()
            .take(n)
            .zip(nums.iter().skip(n))
            .flat_map(|(&x, &y)| vec![x, y].into_iter())
            .collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_shuffle_2() {
        assert_eq!(Solution::shuffle(vec![1, 1, 2, 2], 2), vec![1, 2, 1, 2]);
    }

    #[test]
    fn test_shuffle_3() {
        assert_eq!(
            Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3),
            vec![2, 3, 5, 4, 1, 7]
        );
    }

    #[test]
    fn test_shuffle_4() {
        assert_eq!(
            Solution::shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4),
            vec![1, 4, 2, 3, 3, 2, 4, 1]
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::shuffle(Vec::new(), 0), Vec::<i32>::new());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4));
    }
}
