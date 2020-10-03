/*
Problem 93. Restore IP Addresses
================================

https://leetcode.com/problems/restore-ip-addresses/

Given a string s containing only digits, return all possible valid IP addresses that can be
obtained from s. You can return them in any order.

A valid IP address consists of exactly four integers, each integer is between 0 and 255, separated
by single dots and cannot have leading zeros. For example, "0.1.2.201" and "192.168.1.1" are valid
IP addresses and "0.011.255.245", "192.168.1.312" and "192.168@1.1" are invalid IP addresses.

Example 1:

    Input: s = "25525511135"
    Output: ["255.255.11.135","255.255.111.35"]

Example 2:

    Input: s = "0000"
    Output: ["0.0.0.0"]

Example 3:

    Input: s = "1111"
    Output: ["1.1.1.1"]

Example 4:

    Input: s = "010010"
    Output: ["0.10.0.10","0.100.1.0"]

Example 5:

    Input: s = "101023"
    Output: ["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]

Constraints:

    0 <= s.length <= 3000
    s consists of digits only.
*/

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut ans = Vec::new();
        Self::extract_ip_addr(&s, &mut Vec::new(), &mut ans);
        ans
    }

    fn extract_ip_addr<'a>(s: &'a str, cur_addr: &mut Vec<&'a str>, result: &mut Vec<String>) {
        match (s.is_empty(), cur_addr.len() < 4) {
            (true, false) => result.push(cur_addr.join(".")),
            (false, true) => {
                for dlen in 1..=3.min(s.len()) {
                    let (head, tail) = (s.get(..dlen), s.get(dlen..));
                    if head
                        .filter(|s| s.len() == 1 || !s.starts_with('0'))
                        .filter(|s| s.parse::<u16>().map(|d| d < 256) == Ok(true))
                        .is_some()
                    {
                        cur_addr.push(head.unwrap());
                        Self::extract_ip_addr(tail.unwrap_or(&""), cur_addr, result);
                        cur_addr.pop();
                    }
                }
            }
            _ => (),
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
            Solution::restore_ip_addresses("25525511135".to_string()),
            vec!["255.255.11.135".to_owned(), "255.255.111.35".to_owned(),]
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::restore_ip_addresses("0000".to_string()),
            vec!["0.0.0.0".to_owned()]
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::restore_ip_addresses("1111".to_string()),
            vec!["1.1.1.1".to_owned()]
        );
    }

    #[test]
    fn test_case4() {
        assert_eq!(
            Solution::restore_ip_addresses("010010".to_string()),
            vec!["0.10.0.10".to_owned(), "0.100.1.0".to_owned()]
        );
    }

    #[test]
    fn test_case5() {
        assert_eq!(
            Solution::restore_ip_addresses("101023".to_string()),
            vec![
                "1.0.10.23".to_owned(),
                "1.0.102.3".to_owned(),
                "10.1.0.23".to_owned(),
                "10.10.2.3".to_owned(),
                "101.0.2.3".to_owned(),
            ]
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(
            Solution::restore_ip_addresses("".to_owned()),
            Vec::<String>::new()
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::restore_ip_addresses("25525511135".to_owned());
        });
    }
}
