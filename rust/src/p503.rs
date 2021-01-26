/*
Problem 503. Next Greater Element II
====================================

https://leetcode.com/problems/next-greater-element-ii/

Given a circular array (the next element of the last element is the first element of the array),
print the Next Greater Number for every element. The Next Greater Number of a number x is the first
greater number to its traversing-order next in the array, which means you could search circularly
to find its next greater number. If it doesn't exist, output -1 for this number.

Example 1:
    Input: [1,2,1]
    Output: [2,-1,2]
    Explanation: The first 1's next greater number is 2;
    The number 2 can't find next greater number;
    The second 1's next greater number needs to search circularly, which is also 2.

Note: The length of given array won't exceed 10000.
*/

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut num_buf = Vec::new();
        let mut num_max = std::i32::MIN;
        let mut ans = vec![-1; len];
        for i in 0..len * 2 {
            let (first_cycle, i) = (i < len, i % len);
            let n = nums[i];
            num_max = num_max.max(n);
            while num_buf.last().filter(|&&idx| nums[idx] < n).is_some() {
                if let Some(idx) = num_buf.pop() {
                    ans[idx] = n;
                }
            }
            if first_cycle {
                // first cycle
                num_buf.push(i);
            } else if n == num_max {
                // second cycle, and time to early stop
                break;
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
    fn test_case1() {
        assert_eq!(
            Solution::next_greater_elements(vec![1, 2, 1]),
            vec![2, -1, 2]
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::next_greater_elements(vec![1, 2, 3, 6, 2, 3, 5, 2, 1, 5, 2]),
            vec![2, 3, 6, -1, 3, 5, 6, 5, 5, 6, 3]
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::next_greater_elements(vec![-1, -2, -1, 2, 2, 1, 1]),
            vec![2, -1, 2, -1, -1, 2, 2]
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::next_greater_elements(Vec::new()), Vec::new());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::next_greater_elements(vec![1, 2, 3, 6, 2, 3, 5, 2, 1, 5, 2]));
    }
}
