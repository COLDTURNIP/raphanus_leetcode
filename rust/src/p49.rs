/*
Problem 21. Merge Two Sorted Lists
==================================

https://leetcode.com/problems/group-anagrams/

Given an array of strings strs, group the anagrams together. You can return the answer in any order.

An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase,
typically using all the original letters exactly once.

Example 1:

    Input: strs = ["eat","tea","tan","ate","nat","bat"]
    Output: [["bat"],["nat","tan"],["ate","eat","tea"]]

Example 2:

    Input: strs = [""]
    Output: [[""]]

Example 3:

    Input: strs = ["a"]
    Output: [["a"]]
*/

use std::collections::BTreeMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups = BTreeMap::<Vec<char>, Vec<String>>::new();
        for word in strs.into_iter() {
            let key: Vec<char> = {
                let mut key_vec = word.chars().collect::<Vec<_>>();
                key_vec.sort_unstable();
                key_vec
            };
            groups.entry(key).or_insert_with(Vec::new).push(word);
        }
        groups.into_iter().map(|(_, v)| v).collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    #[test]
    fn test_common() {
        assert_eq!(
            Solution::group_anagrams(vec![
                "eat".to_owned(),
                "tea".to_owned(),
                "tan".to_owned(),
                "ate".to_owned(),
                "nat".to_owned(),
                "bat".to_owned()
            ]),
            vec![
                vec!["bat".to_owned()],
                vec!["eat".to_owned(), "tea".to_owned(), "ate".to_owned()],
                vec!["tan".to_owned(), "nat".to_owned()]
            ]
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(
            Solution::group_anagrams(vec!["".to_owned()]),
            vec![vec!["".to_owned()]]
        );
        assert_eq!(
            Solution::group_anagrams(Vec::new()),
            Vec::<Vec<String>>::new()
        );
    }

    #[test]
    fn test_single() {
        assert_eq!(
            Solution::group_anagrams(vec!["a".to_owned()]),
            vec![vec!["a".to_owned()]]
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::group_anagrams(vec![
                "eat".to_owned(),
                "tea".to_owned(),
                "tan".to_owned(),
                "ate".to_owned(),
                "nat".to_owned(),
                "bat".to_owned(),
            ])
        });
    }
}
