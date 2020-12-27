/*
Problem 151. Reverse Words in a String
======================================

https://leetcode.com/problems/reverse-words-in-a-string/

Given an input string s, reverse the order of the words.

A word is defined as a sequence of non-space characters. The words in s will be separated by at
least one space.

Return a string of the words in reverse order concatenated by a single space.

Note that s may contain leading or trailing spaces or multiple spaces between two words. The
returned string should only have a single space separating the words. Do not include any extra
spaces.

Example 1:

    Input: s = "the sky is blue"
    Output: "blue is sky the"

Example 2:

    Input: s = "  hello world  "
    Output: "world hello"
    Explanation: Your reversed string should not contain leading or trailing spaces.

Example 3:

    Input: s = "a good   example"
    Output: "example good a"
    Explanation: You need to reduce multiple spaces between two words to a single space in the
    reversed string.

Example 4:

    Input: s = "  Bob    Loves  Alice   "
    Output: "Alice Loves Bob"

Example 5:

    Input: s = "Alice does not even like bob"
    Output: "bob like even not does Alice"

Constraints:

    - 1 <= s.length <= 104
    - s contains English letters (upper-case and lower-case), digits, and spaces ' '.
    - There is at least one word in s.

Follow up:

    Could you solve it in-place with O(1) extra space?
*/

impl Solution {
    pub fn reverse_words(mut s: String) -> String {
        // Byte-wise editing of a String in Rust is unsafe. It is safe for us becase of the input
        // constraints.
        unsafe {
            // First of all, reverse the whole sentence and trim it
            let v = s.as_mut_vec();
            let mut len = v.len();
            while len > 0 {
                if v[len - 1] != b' ' {
                    break;
                }
                len -= 1;
            }
            v.truncate(len);
            v.reverse();

            let mut wi = 0;
            let mut ci = 0;
            while let Some(start) = (ci..len).find(|&i| v[i] != b' ') {
                // Step 0: prepend the delimiter if need
                if wi > 0 {
                    v[wi] = b' ';
                    wi += 1;
                }

                // Step 1: find the end of current word, and reverse in place
                let end = (start..len).find(|&i| v[i] == b' ').unwrap_or(len);
                let (mut li, mut ri) = (start, end - 1);
                while li < ri {
                    v.swap(li, ri);
                    li += 1;
                    ri -= 1;
                }

                // Step 2: move the word to the correct place
                for cur in start..end {
                    v[wi] = v[cur];
                    wi += 1;
                }
                ci = end;
            }
            s.truncate(wi);
        }
        s
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_trim_space() {
        assert_eq!(
            Solution::reverse_words("  hello world   ".to_owned()),
            "world hello".to_owned()
        );
    }

    #[test]
    fn test_reduce_space() {
        assert_eq!(
            Solution::reverse_words("  Bob    Loves  Alice    ".to_owned()),
            "Alice Loves Bob".to_owned()
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::reverse_words("      ".to_owned()), "".to_owned());
        assert_eq!(Solution::reverse_words(String::new()), String::new());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::reverse_words("  Bob    Loves  Alice    ".to_owned()));
    }
}
