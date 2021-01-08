/*
Problem 583. Delete Operation for Two Strings
=============================================

Given two words word1 and word2, find the minimum number of steps required to make word1 and word2
the same, where in each step you can delete one character in either string.

Example 1:

    Input: "sea", "eat"
    Output: 2
    Explanation: You need one step to make "sea" to "ea" and another step to make "eat" to "ea".

Note:
    - The length of given words won't exceed 500.
    - Characters in given words can only be lower-case letters.
*/

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1: Vec<char> = word1.chars().collect();
        let word2: Vec<char> = word2.chars().collect();
        let mut match_len = vec![0; word1.len()];
        let mut last_match_len = vec![0; word1.len()];
        for c2 in word2.iter() {
            std::mem::swap(&mut match_len, &mut last_match_len);
            let mut w1_iter = word1.iter().enumerate();
            if let Some((_, c1)) = w1_iter.next() {
                match_len[0] = if c1 == c2 { 1 } else { last_match_len[0] };
            }
            for (i1, c1) in w1_iter {
                if c1 == c2 {
                    match_len[i1] = last_match_len[i1 - 1] + 1;
                } else {
                    match_len[i1] = last_match_len[i1].max(match_len[i1 - 1]);
                }
            }
        }
        let common_len = match_len.last().cloned().unwrap_or(0);
        (word1.len() - common_len + word2.len() - common_len) as i32
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
            Solution::min_distance(String::from("sea"), String::from("eat")),
            2
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::min_distance(String::from("ab"), String::from("a")),
            1
        );
        assert_eq!(
            Solution::min_distance(String::from("a"), String::from("ab")),
            1
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::min_distance(String::from("food"), String::from("money")),
            7
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(
            Solution::min_distance(String::from("food"), String::new()),
            4
        );
        assert_eq!(
            Solution::min_distance(String::new(), String::from("hello")),
            5
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::min_distance(String::from("sea"), String::from("eat")));
    }
}
