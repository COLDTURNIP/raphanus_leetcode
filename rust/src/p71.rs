/*
Problem 71. Simplify Path
=========================

https://leetcode.com/problems/simplify-path/

Given an absolute path for a file (Unix-style), simplify it. Or in other words, convert it to the
canonical path.

In a UNIX-style file system, a period . refers to the current directory. Furthermore, a double
period .. moves the directory up a level.

Note that the returned canonical path must always begin with a slash /, and there must be only a
single slash / between two directory names. The last directory name (if it exists) must not end
with a trailing /. Also, the canonical path must be the shortest string representing the absolute
path.



Example 1:

    Input: "/home/"
    Output: "/home"
    Explanation: Note that there is no trailing slash after the last directory name.

Example 2:

    Input: "/../"
    Output: "/"
    Explanation: Going one level up from the root directory is a no-op, as the root level is the
                 highest level you can go.

Example 3:

    Input: "/home//foo/"
    Output: "/home/foo"
    Explanation: In the canonical path, multiple consecutive slashes are replaced by a single one.

Example 4:

    Input: "/a/./b/../../c/"
    Output: "/c"

Example 5:

    Input: "/a/../../b/../c//.//"
    Output: "/c"

Example 6:

    Input: "/a//b////c/d//././/.."
    Output: "/a/b/c"
*/

impl Solution {
    const CURRENT_DIR: &'static str = ".";
    const PARENT_DIR: &'static str = "..";

    pub fn simplify_path(path: String) -> String {
        let parts = path.split('/').filter(|p| !p.is_empty());
        let mut result = Vec::new();
        for part in parts {
            if part == Self::PARENT_DIR {
                result.pop();
                result.pop();
            } else if part != Self::CURRENT_DIR {
                result.push("/");
                result.push(part);
            }
        }
        if result.is_empty() {
            "/".to_owned()
        } else {
            result.concat()
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_tail_slash() {
        assert_eq!(
            Solution::simplify_path("/home/".to_owned()),
            "/home".to_owned()
        );
    }

    #[test]
    fn test_parent_folder_to_root() {
        assert_eq!(Solution::simplify_path("/../".to_owned()), "/".to_owned());
    }

    #[test]
    fn test_multiple_slash() {
        assert_eq!(
            Solution::simplify_path("/home//foo/".to_owned()),
            "/home/foo".to_owned()
        );
    }

    #[test]
    fn test_current_folder() {
        assert_eq!(
            Solution::simplify_path("/a/../../b/../c//.//".to_owned()),
            "/c".to_owned()
        );
    }

    #[test]
    fn test_abc() {
        assert_eq!(
            Solution::simplify_path("/a//b////c/d//././/..".to_owned()),
            "/a/b/c".to_owned()
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::simplify_path("/a//b////c/d//././/..".to_owned()));
    }
}
