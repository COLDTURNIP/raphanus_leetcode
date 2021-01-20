/*
Problem 150. Evaluate Reverse Polish Notation
=============================================

https://leetcode.com/problems/evaluate-reverse-polish-notation/

Evaluate the value of an arithmetic expression in Reverse Polish Notation.

Valid operators are +, -, *, /. Each operand may be an integer or another expression.

Note:

    - Division between two integers should truncate toward zero.
    - The given RPN expression is always valid. That means the expression would always evaluate to
      a result and there won't be any divide by zero operation.

Example 1:

    Input: ["2", "1", "+", "3", "*"]
    Output: 9
    Explanation: ((2 + 1) * 3) = 9

Example 2:

    Input: ["4", "13", "5", "/", "+"]
    Output: 6
    Explanation: (4 + (13 / 5)) = 6

Example 3:

    Input: ["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
    Output: 22
    Explanation:
      ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
    = ((10 * (6 / (12 * -11))) + 17) + 5
    = ((10 * (6 / -132)) + 17) + 5
    = ((10 * 0) + 17) + 5
    = (0 + 17) + 5
    = 17 + 5
    = 22
*/

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut buf = Vec::new();
        let pop2 = |buf: &mut Vec<i32>| (buf.pop().unwrap(), buf.pop().unwrap());
        for token in tokens {
            match token.as_str() {
                "+" => {
                    let (n2, n1) = pop2(&mut buf);
                    buf.push(n1 + n2);
                }
                "-" => {
                    let (n2, n1) = pop2(&mut buf);
                    buf.push(n1 - n2);
                }
                "*" => {
                    let (n2, n1) = pop2(&mut buf);
                    buf.push(n1 * n2);
                }
                "/" => {
                    let (n2, n1) = pop2(&mut buf);
                    buf.push(n1 / n2);
                }
                _ => buf.push(token.parse().unwrap()),
            }
        }
        buf.pop().unwrap()
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
            Solution::eval_rpn(
                ["2", "1", "+", "3", "*"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            ),
            9
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::eval_rpn(
                ["4", "13", "5", "/", "+"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            ),
            6
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::eval_rpn(
                ["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            ),
            22
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::eval_rpn(
                [
                    "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            )
        });
    }
}
