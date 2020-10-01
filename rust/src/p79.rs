/*
Problem 79. Word Search
=======================

https://leetcode.com/problems/word-search/

Given a 2D board and a word, find if the word exists in the grid.

The word can be constructed from letters of sequentially adjacent cell, where "adjacent" cells are
those horizontally or vertically neighboring. The same letter cell may not be used more than once.

Example:

    board =
    [
      ['A','B','C','E'],
      ['S','F','C','S'],
      ['A','D','E','E']
    ]

    Given word = "ABCCED", return true.
    Given word = "SEE", return true.
    Given word = "ABCB", return false.

Constraints:

    - board and word consists only of lowercase and uppercase English letters.
    - 1 <= board.length <= 200
    - 1 <= board[i].length <= 200
    - 1 <= word.length <= 10^3
*/

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        if board.is_empty() || board.first().map_or(0, |row| row.len()) == 0 {
            return word.is_empty();
        }
        let (x_len, y_len) = (board[0].len(), board.len());
        Self::traverse(
            &mut board,
            &(x_len as i32, y_len as i32),
            &word.chars().collect::<Vec<char>>(),
            0,
            &(0, 0),
        )
    }

    fn traverse(
        map: &mut [Vec<char>],
        &(x_len, y_len): &(i32, i32),
        char_list: &[char],
        char_idx: usize,
        &(x, y): &(i32, i32),
    ) -> bool {
        if char_idx >= char_list.len() {
            true
        } else if char_idx == 0 {
            let cur_c = char_list[char_idx];
            for y in 0..map.len() {
                for x in 0..map[y].len() {
                    if cur_c != map[y][x] {
                        continue;
                    }
                    map[y][x] = ' ';
                    if Self::traverse(
                        map,
                        &(x_len, y_len),
                        char_list,
                        char_idx + 1,
                        &(x as i32, y as i32),
                    ) {
                        return true;
                    }
                    map[y][x] = cur_c
                }
            }
            false
        } else {
            let cur_c = char_list[char_idx];
            let next_pos = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
            for (x, y) in next_pos
                .into_iter()
                .filter(|&(x, y)| x >= 0 && x < x_len && y >= 0 && y < y_len)
            {
                if cur_c != map[y as usize][x as usize] {
                    continue;
                }
                map[y as usize][x as usize] = ' ';
                if Self::traverse(map, &(x_len, y_len), char_list, char_idx + 1, &(x, y)) {
                    return true;
                }
                map[y as usize][x as usize] = cur_c;
            }
            false
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_found1() {
        assert!(Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            "ABCCED".to_owned()
        ));
    }

    #[test]
    fn test_found2() {
        assert!(Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            "SEE".to_owned()
        ));
    }

    #[test]
    fn test_found3() {
        assert!(Solution::exist(vec![vec!['a', 'a'],], "aa".to_owned()));
    }

    #[test]
    fn test_not_found() {
        assert!(!Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            "ABCB".to_owned()
        ));
    }

    #[test]
    fn test_not_found_exceed() {
        assert!(!Solution::exist(vec![vec!['a'],], "ab".to_owned()));
    }

    #[test]
    fn test_empty() {
        assert!(Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            "".to_owned()
        ));
        assert!(Solution::exist(Vec::new(), "".to_owned()));
        assert!(!Solution::exist(Vec::new(), "AAB".to_owned()));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                "ABCCED".to_owned(),
            );
        });
    }
}
