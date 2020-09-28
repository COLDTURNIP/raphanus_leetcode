/*
Problem 65. Valid Number
========================

https://leetcode.com/problems/valid-number/

Validate if a given string can be interpreted as a decimal number.

Some examples:
    "0" => true
    " 0.1 " => true
    "abc" => false
    "1 a" => false
    "2e10" => true
    " -90e3   " => true
    " 1e" => false
    "e3" => false
    " 6e-1" => true
    " 99e2.5 " => false
    "53.5e93" => true
    " --6 " => false
    "-+3" => false
    "95a54e53" => false

Note: It is intended for the problem statement to be ambiguous. You should gather all requirements
up front before implementing one. However, here is a list of characters that can be in a valid
decimal number:

    Numbers 0-9
    Exponent - "e"
    Positive/negative sign - "+"/"-"
    Decimal point - "."

Of course, the context of these characters also matters in the input.
*/

impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut exparts = s.trim().split('e');
        let mut ans = exparts.next().map(|part| Self::is_float(part)) == Some(true);
        ans &= exparts.next().map(|part| Self::is_int(part)) != Some(false);
        ans & exparts.next().is_none()
    }

    // workaround for Leetcode: if separate the sign removal logic to a dedicated utility function,
    // it would impact the performance since there's no inline optimization.

    fn is_float(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        let s = match s.chars().next() {
            Some('+') | Some('-') => s.get(1..).unwrap_or(&""),
            _ => s,
        };
        let mut parts = s.split('.');
        let ans = match (parts.next(), parts.next()) {
            (Some(first), Some("")) => Self::is_uint(first),
            (Some(""), Some(second)) => Self::is_uint(second),
            (Some(first), Some(second)) => Self::is_uint(first) && Self::is_uint(second),
            (Some(first), None) => Self::is_uint(first),
            _ => false,
        };
        ans & parts.next().is_none()
    }

    fn is_int(s: &str) -> bool {
        let s = match s.chars().next() {
            Some('+') | Some('-') => s.get(1..).unwrap_or(&""),
            _ => s,
        };
        !s.is_empty() && s.chars().all(|c| c.is_digit(10))
    }

    fn is_uint(s: &str) -> bool {
        !s.is_empty() && s.chars().all(|c| c.is_digit(10))
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    #[test]
    fn test_zero() {
        assert!(Solution::is_number("0".to_owned()));
    }

    #[test]
    fn test_float() {
        assert!(Solution::is_number(" 0.1".to_owned()));
        assert!(Solution::is_number(" .1".to_owned()));
    }

    #[test]
    fn test_abc() {
        assert!(!Solution::is_number("abc".to_owned()));
    }

    #[test]
    fn test_space_inside() {
        assert!(!Solution::is_number("1 a".to_owned()));
    }

    #[test]
    fn test_exp() {
        assert!(Solution::is_number("2e10".to_owned()));
    }

    #[test]
    fn test_neg_with_exp() {
        assert!(Solution::is_number(" -90e3".to_owned()));
    }

    #[test]
    fn test_empty_exp() {
        assert!(!Solution::is_number("  1e".to_owned()));
    }

    #[test]
    fn test_exp_without_fraction() {
        assert!(!Solution::is_number("e3".to_owned()));
    }

    #[test]
    fn test_neg_exp() {
        assert!(Solution::is_number("  6e-1".to_owned()));
    }

    #[test]
    fn test_exp_not_integer() {
        assert!(!Solution::is_number(" 99e2.5".to_owned()));
    }

    #[test]
    fn test_float_with_exp() {
        assert!(Solution::is_number("53.5e93".to_owned()));
    }

    #[test]
    fn test_multiple_neg_sign() {
        assert!(!Solution::is_number(" --6".to_owned()));
    }

    #[test]
    fn test_combined_neg_sign() {
        assert!(!Solution::is_number("-+3".to_owned()));
    }

    #[test]
    fn test_invalid_char_inside() {
        assert!(!Solution::is_number("95a54e53".to_owned()));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::is_number("-789234.652344e-24349087".to_owned()));
    }
}
