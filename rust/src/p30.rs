/*
Problem 30. Substring with Concatenation of All Words
=====================================================

https://leetcode.com/problems/substring-with-concatenation-of-all-words/

You are given a string s and an array of strings words of the same length. Return all starting
indices of substring(s) in s that is a concatenation of each word in words exactly once, in any
order, and without any intervening characters.

You can return the answer in any order.

Example 1:

    Input: s = "barfoothefoobarman", words = ["foo","bar"]
    Output: [0,9]
    Explanation: Substrings starting at index 0 and 9 are "barfoo" and "foobar" respectively.
    The output order does not matter, returning [9,0] is fine too.

Example 2:

    Input: s = "wordgoodgoodgoodbestword", words = ["word","good","best","word"]
    Output: []

Example 3:

    Input: s = "barfoofoobarthefoobarman", words = ["bar","foo","the"]
    Output: [6,9,12]

Constraints:

    1 <= s.length <= 104
    s consists of lower-case English letters.
    1 <= words.length <= 5000
    1 <= words[i].length <= 30
    words[i] consists of lower-case English letters.
*/

use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let s_len = s.len();
        let ws_len = words.len();
        if s_len == 0 || ws_len == 0 || words[0].is_empty() {
            return Vec::new();
        }
        let w_len = words[0].len();

        // expected counts of each word
        let expect = {
            let mut cnt = HashMap::new();
            words.iter().for_each(|w| {
                cnt.entry(w.as_str())
                    .and_modify(|c| *c += 1)
                    .or_insert(1_usize);
            });
            cnt
        };
        let mut ans = Vec::new();

        // counting context by offset
        struct Context<'a> {
            ok_cnt: usize,
            ws_cnt: HashMap<&'a str, usize>,
        };
        let mut context = (0..w_len)
            .map(|_| Context {
                ok_cnt: 0,
                ws_cnt: words.iter().map(|w| (w.as_str(), 0)).collect(),
            })
            .collect::<Vec<Context>>();

        for idx in 0..=s_len {
            let new_word = if let Some(wref) = s.get(idx..idx + w_len) {
                wref
            } else {
                break;
            };
            let ctx = &mut context[idx % w_len];

            // inserting new word into window
            if let Some(&expect_cnt) = expect.get(new_word) {
                let mut mut_ok = None;
                ctx.ws_cnt.entry(new_word).and_modify(|c| {
                    if *c == expect_cnt {
                        mut_ok = Some(false);
                    }
                    *c += 1;
                    if *c == expect_cnt {
                        mut_ok = Some(true);
                    }
                });
                match mut_ok {
                    Some(true) => ctx.ok_cnt += 1,
                    Some(false) => ctx.ok_cnt -= 1,
                    None => (),
                }
            }

            // removing old word from window
            if let Some(old_word) = idx
                .checked_sub(w_len * ws_len)
                .and_then(|i| s.get(i..i + w_len))
            {
                let mut mut_ok = None;
                ctx.ws_cnt.entry(old_word).and_modify(|c| {
                    let expect_cnt = *expect.get(old_word).unwrap();
                    if *c == expect_cnt {
                        mut_ok = Some(false);
                    }
                    *c -= 1;
                    if *c == expect_cnt {
                        mut_ok = Some(true);
                    }
                });
                match mut_ok {
                    Some(true) => ctx.ok_cnt += 1,
                    Some(false) => ctx.ok_cnt -= 1,
                    None => (),
                }
            }

            if ctx.ok_cnt == expect.len() {
                ans.push((idx - w_len * (ws_len - 1)) as i32);
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
    fn test_found() {
        assert_eq!(
            Solution::find_substring(
                "barfoofoobarthefoobarman".to_owned(),
                vec!["bar".to_owned(), "foo".to_owned(), "the".to_owned()]
            ),
            vec![6, 9, 12]
        );
    }

    #[test]
    fn test_found2() {
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_owned(),
                vec![
                    "word".to_owned(),
                    "good".to_owned(),
                    "best".to_owned(),
                    "good".to_owned()
                ]
            ),
            vec![8]
        );
    }

    #[test]
    fn test_not_found() {
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_owned(),
                vec![
                    "word".to_owned(),
                    "good".to_owned(),
                    "best".to_owned(),
                    "word".to_owned()
                ]
            ),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(
            Solution::find_substring(
                String::new(),
                vec!["bar".to_owned(), "foo".to_owned(), "the".to_owned()]
            ),
            Vec::<i32>::new()
        );
        assert_eq!(
            Solution::find_substring(String::new(), Vec::new()),
            Vec::<i32>::new()
        );
        assert_eq!(
            Solution::find_substring(String::new(), vec![String::new()]),
            Vec::<i32>::new()
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_owned(),
                vec!["bar".to_owned(), "foo".to_owned(), "the".to_owned()],
            )
        });
    }
}
