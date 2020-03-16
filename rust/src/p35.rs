/*
35. Search Insert Position
=======================================

https://leetcode.com/problems/search-insert-position/

Given a sorted array and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

You may assume no duplicates in the array.

Example 1:

    Input: [1,3,5,6], 5
    Output: 2

Example 2:

    Input: [1,3,5,6], 2
    Output: 1

Example 3:

    Input: [1,3,5,6], 7
    Output: 4

Example 4:

    Input: [1,3,5,6], 0
    Output: 0

*/

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;
        if nums.is_empty() || target <= nums[0] {
            return 0
        }
        let mut low = 0usize;
        let mut high = nums.len();
        while low < high {
            let mid = (low + high) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Equal => {
                    low = mid;
                    break;
                },
                Ordering::Less => low = mid+1,
                Ordering::Greater => high = mid,
            }
        }
        low as i32
    }
}

pub struct Solution;

mod internal {
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6, 11, 19, 31], 16), 5);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6, 11, 19, 31], 11), 4);
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use super::Solution;

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::search_insert(vec![1, 3, 5, 6], 4));
    }
}
