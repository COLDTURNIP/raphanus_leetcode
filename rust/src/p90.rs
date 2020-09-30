/*
Problem 90. Subsets II
======================

https://leetcode.com/problems/subsets-ii/

Given a collection of integers that might contain duplicates, nums, return all possible subsets
(the power set).

Note: The solution set must not contain duplicate subsets.

Example:

    Input: [1,2,2]
    Output:
    [
      [2],
      [1],
      [1,2,2],
      [2,2],
      [1,2],
      []
    ]
*/

use std::collections::HashMap;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let cnt: HashMap<i32, usize> = nums.into_iter().fold(HashMap::new(), |mut cnt, n| {
            *cnt.entry(n).or_insert(0) += 1;
            cnt
        });
        let num_list: Vec<&i32> = cnt.keys().collect();
        let mut ans = Vec::new();
        Self::subsets(&num_list, 0, &cnt, &mut Vec::new(), &mut ans);
        ans
    }

    fn subsets(
        num_list: &[&i32],
        num_idx: usize,
        num_cnt: &HashMap<i32, usize>,
        cur_ans: &mut Vec<i32>,
        final_ans: &mut Vec<Vec<i32>>,
    ) {
        if let Some(&&num) = num_list.get(num_idx) {
            Self::subsets(num_list, num_idx + 1, num_cnt, cur_ans, final_ans);
            let cur_ans_len = cur_ans.len();
            for _ in 1..=num_cnt[&num] {
                cur_ans.push(num);
                Self::subsets(num_list, num_idx + 1, num_cnt, cur_ans, final_ans);
            }
            cur_ans.truncate(cur_ans_len);
        } else {
            final_ans.push(cur_ans.clone());
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;
    use std::collections::HashSet;

    fn check(input: Vec<i32>, expect: Vec<Vec<i32>>) {
        let exact: HashSet<Vec<i32>> = Solution::subsets_with_dup(input).into_iter().collect();
        let expect: HashSet<Vec<i32>> = expect.into_iter().collect();
        assert_eq!(exact, expect);
    }

    #[test]
    fn test_122() {
        check(
            vec![1, 2, 2],
            vec![
                Vec::new(),
                vec![1],
                vec![2],
                vec![1, 2],
                vec![1, 2, 2],
                vec![2, 2],
            ],
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::subsets_with_dup(vec![1, 2, 2, 2, 2, 3, 6, 6, 6, 6, 6, 6, 7, 7, 8, 8]);
        });
    }
}
