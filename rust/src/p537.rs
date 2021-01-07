/*
Problem 537. Complex Number Multiplication
==========================================

Given two strings representing two complex numbers.

You need to return a string representing their multiplication. Note i2 = -1 according to the
definition.

Example 1:

    Input: "1+1i", "1+1i"
    Output: "0+2i"
    Explanation: (1 + i) * (1 + i) = 1 + i2 + 2 * i = 2i, and you need convert it to the form of
    0+2i.

Example 2:

    Input: "1+-1i", "1+-1i"
    Output: "0+-2i"
    Explanation: (1 - i) * (1 - i) = 1 + i2 - 2 * i = -2i, and you need convert it to the form of
    0+-2i.

Note:

    - The input strings will not have extra blank.
    - The input strings will be given in the form of a+bi, where the integer a and b will both belong
      to the range of [-100, 100]. And the output should be also in this form.
*/

impl Solution {
    pub fn complex_number_multiply(a: String, b: String) -> String {
        let (ra, ia): (i32, i32) = {
            let mut parts = a[..a.len() - 1].split('+');
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        };
        let (rb, ib): (i32, i32) = {
            let mut parts = b[..b.len() - 1].split('+');
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        };
        let real = ra * rb - ia * ib;
        let img = ra * ib + ia * rb;
        format!("{}+{}i", real, img)
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
            Solution::complex_number_multiply("1+1i".to_owned(), "1+1i".to_owned()),
            "0+2i".to_string()
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::complex_number_multiply("1+-1i".to_owned(), "1+-1i".to_owned()),
            "0+-2i".to_string()
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::complex_number_multiply("1+1i".to_owned(), "1+1i".to_owned()));
    }
}
