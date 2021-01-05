/*
Problem 165. Compare Version Numbers
====================================

Given two version numbers, version1 and version2, compare them.

Version numbers consist of one or more revisions joined by a dot '.'. Each revision consists of
digits and may contain leading zeros. Every revision contains at least one character. Revisions are
0-indexed from left to right, with the leftmost revision being revision 0, the next revision being
revision 1, and so on. For example 2.5.33 and 0.1 are valid version numbers.

To compare version numbers, compare their revisions in left-to-right order. Revisions are compared
using their integer value ignoring any leading zeros. This means that revisions 1 and 001 are
considered equal. If a version number does not specify a revision at an index, then treat the
revision as 0. For example, version 1.0 is less than version 1.1 because their revision 0s are the
same, but their revision 1s are 0 and 1 respectively, and 0 < 1.

Return the following:

    - If version1 < version2, return -1.
    - If version1 > version2, return 1.
    - Otherwise, return 0.

Example 1:

    Input: version1 = "1.01", version2 = "1.001"
    Output: 0
    Explanation: Ignoring leading zeroes, both "01" and "001" represent the same integer "1".

Example 2:

    Input: version1 = "1.0", version2 = "1.0.0"
    Output: 0
    Explanation: version1 does not specify revision 2, which means it is treated as "0".

Example 3:

    Input: version1 = "0.1", version2 = "1.1"
    Output: -1
    Explanation: version1's revision 0 is "0", while version2's revision 0 is "1". 0 < 1, so
    version1 < version2.

Example 4:

    Input: version1 = "1.0.1", version2 = "1"
    Output: 1

Example 5:

    Input: version1 = "7.5.2.4", version2 = "7.5.3"
    Output: -1

Constraints:

    - 1 <= version1.length, version2.length <= 500
    - version1 and version2 only contain digits and '.'.
    - version1 and version2 are valid version numbers.
    - All the given revisions in version1 and version2 can be stored in a 32-bit integer.
*/

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut parts1_iter = version1.split('.').flat_map(|part| part.parse::<i32>());
        let mut parts2_iter = version2.split('.').flat_map(|part| part.parse::<i32>());
        loop {
            match (parts1_iter.next(), parts2_iter.next()) {
                (Some(v1), Some(v2)) => match v1.cmp(&v2) {
                    std::cmp::Ordering::Less => break -1,
                    std::cmp::Ordering::Greater => break 1,
                    _ => {}
                },
                (Some(v1), None) if v1 > 0 => break 1,
                (None, Some(v2)) if v2 > 0 => break -1,
                (None, None) => break 0,
                _ => {}
            }
        }
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
            Solution::compare_version("1.01".to_owned(), "1.001".to_owned()),
            0
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::compare_version("1.0".to_owned(), "1.0.0".to_owned()),
            0
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::compare_version("0.1".to_owned(), "1.1".to_owned()),
            -1
        );
    }

    #[test]
    fn test_case4() {
        assert_eq!(
            Solution::compare_version("1.0.1".to_owned(), "1".to_owned()),
            1
        );
    }

    #[test]
    fn test_case5() {
        assert_eq!(
            Solution::compare_version("7.5.2.4".to_owned(), "7.5.3".to_owned()),
            -1
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::compare_version("7.5.2.4".to_owned(), "7.5.3".to_owned()));
    }
}
