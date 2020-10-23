/*
Problem 228. Summary Ranges
===========================

https://leetcode.com/problems/summary-ranges/

You are given a sorted unique integer array nums.

Return the smallest sorted list of ranges that cover all the numbers in the array exactly. That is,
each element of nums is covered by exactly one of the ranges, and there is no integer x such that x
is in one of the ranges but not in nums.

Each range [a,b] in the list should be output as:

    "a->b" if a != b
    "a" if a == b

Example 1:

    Input: nums = [0,1,2,4,5,7]
    Output: ["0->2","4->5","7"]
    Explanation: The ranges are:
        [0,2] --> "0->2"
        [4,5] --> "4->5"
        [7,7] --> "7"

Example 2:

    Input: nums = [0,2,3,4,6,8,9]
    Output: ["0","2->4","6","8->9"]
    Explanation: The ranges are:
        [0,0] --> "0"
        [2,4] --> "2->4"
        [6,6] --> "6"
        [8,9] --> "8->9"

Example 3:

    Input: nums = []
    Output: []

Example 4:

    Input: nums = [-1]
    Output: ["-1"]

Example 5:

    Input: nums = [0]
    Output: ["0"]

Constraints:

    - 0 <= nums.length <= 20
    - -231 <= nums[i] <= 231 - 1
    - All the values of nums are unique.
*/

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut ret = Vec::new();
        let mut iter = nums.into_iter().enumerate();
        let mut push_ans = |start: i32, end: i32| {
            ret.push(if start == end {
                start.to_string()
            } else {
                format!("{}->{}", start, end)
            })
        };
        let mut start;
        let mut end;
        let mut state;
        if let Some((_, first)) = iter.next() {
            start = first;
            end = first;
            state = first;
            for (i, n) in iter {
                let cur_state = n - i as i32;
                if cur_state == state {
                    end = n;
                } else {
                    push_ans(start, end);
                    state = cur_state;
                    start = n;
                    end = n;
                }
            }
            push_ans(start, end);
        }
        ret
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_012457() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            vec!["0->2".to_owned(), "4->5".to_owned(), "7".to_owned()]
        );
    }

    #[test]
    fn test_0234689() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
            vec![
                "0".to_owned(),
                "2->4".to_owned(),
                "6".to_owned(),
                "8->9".to_owned(),
            ]
        );
    }

    #[test]
    fn test_single_negative() {
        assert_eq!(Solution::summary_ranges(vec![-1]), vec!["-1".to_owned()]);
    }

    #[test]
    fn test_single_zero() {
        assert_eq!(Solution::summary_ranges(vec![0]), vec!["0".to_owned()]);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::summary_ranges(Vec::new()), Vec::<String>::new());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]));
    }
}
