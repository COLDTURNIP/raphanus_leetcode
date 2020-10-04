/*
Problem 127. Word Ladder
========================

https://leetcode.com/problems/word-ladder/

Given two words (beginWord and endWord), and a dictionary's word list, find the length of shortest
transformation sequence from beginWord to endWord, such that:

Only one letter can be changed at a time.
Each transformed word must exist in the word list.

Note:

    - Return 0 if there is no such transformation sequence.
    - All words have the same length.
    - All words contain only lowercase alphabetic characters.
    - You may assume no duplicates in the word list.
    - You may assume beginWord and endWord are non-empty and are not the same.

Example 1:

    Input:
    beginWord = "hit",
    endWord = "cog",
    wordList = ["hot","dot","dog","lot","log","cog"]

    Output: 5

    Explanation: As one shortest transformation is "hit" -> "hot" -> "dot" -> "dog" -> "cog",
    return its length 5.

Example 2:

    Input:
    beginWord = "hit"
    endWord = "cog"
    wordList = ["hot","dot","dog","lot","log"]

    Output: 0

    Explanation: The endWord "cog" is not in wordList, therefore no possible transformation.
*/

use std::collections::VecDeque;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        // TODO: Need performance improvement. It is faster than only 61.54% of Rust online
        // submissions.
        let mut word_list: VecDeque<(Vec<char>, bool)> = word_list
            .iter()
            .map(|s| (s.chars().collect::<Vec<char>>(), *s == end_word))
            .collect();
        let begin_word: Vec<char> = begin_word.chars().collect();

        let distance =
            |w1: &[char], w2: &[char]| w1.iter().zip(w2.iter()).filter(|(c1, c2)| c1 != c2).count();

        let mut last_seen = Vec::new();
        let mut new_seen = Vec::new();
        last_seen.push(begin_word);
        let mut level = 1;
        let mut solvable = true;
        while solvable && !last_seen.is_empty() {
            for _ in 0..word_list.len() {
                if let Some((word, end)) = word_list.pop_front() {
                    solvable |= end;
                    match (
                        end,
                        last_seen
                            .iter()
                            .any(|seen| distance(&word, seen.as_slice()) <= 1),
                    ) {
                        (true, true) => return level + 1,
                        (false, true) => {
                            new_seen.push(word);
                        }
                        (_, false) => word_list.push_back((word, end)),
                    }
                } else {
                    unreachable!();
                }
            }
            std::mem::swap(&mut last_seen, &mut new_seen);
            new_seen.truncate(0);
            level += 1;
        }
        0
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_solvable() {
        assert_eq!(
            Solution::ladder_length(
                "hit".to_owned(),
                "cog".to_owned(),
                vec![
                    "hot".to_owned(),
                    "dot".to_owned(),
                    "dog".to_owned(),
                    "lot".to_owned(),
                    "log".to_owned(),
                    "cog".to_owned()
                ]
            ),
            5
        );
    }

    #[test]
    fn test_not_solvable() {
        assert_eq!(
            Solution::ladder_length(
                "hit".to_owned(),
                "cog".to_owned(),
                vec![
                    "hot".to_owned(),
                    "dot".to_owned(),
                    "dog".to_owned(),
                    "lot".to_owned(),
                    "log".to_owned()
                ]
            ),
            0
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(
            Solution::ladder_length("".to_owned(), "".to_owned(), Vec::new()),
            0
        );
        assert_eq!(
            Solution::ladder_length("abcd".to_owned(), "abcd".to_owned(), Vec::new()),
            0
        );
        assert_eq!(
            Solution::ladder_length("abcd".to_owned(), "abce".to_owned(), Vec::new()),
            0
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::ladder_length(
                "hit".to_owned(),
                "cog".to_owned(),
                vec![
                    "hot".to_owned(),
                    "dot".to_owned(),
                    "dog".to_owned(),
                    "lot".to_owned(),
                    "log".to_owned(),
                    "cog".to_owned(),
                ],
            );
        });
    }
}
