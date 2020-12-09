/*
Problem 650. 2 Keys Keyboard
============================

https://leetcode.com/problems/2-keys-keyboard/

Initially on a notepad only one character 'A' is present. You can perform two operations on this
notepad for each step:

    - Copy All: You can copy all the characters present on the notepad (partial copy is not
                allowed).
    - Paste: You can paste the characters which are copied last time.

Given a number n. You have to get exactly n 'A' on the notepad by performing the minimum number of
steps permitted. Output the minimum number of steps to get n 'A'.

Example 1:

    Input: 3
    Output: 3
    Explanation:
    Intitally, we have one character 'A'.
    In step 1, we use Copy All operation.
    In step 2, we use Paste operation to get 'AA'.
    In step 3, we use Paste operation to get 'AAA'.

Note:

    The n will be in the range [1, 1000].
*/

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        if n <= 1 {
            return 0;
        }
        let n = n as usize;
        let mut steps = vec![0; n as usize + 1];
        steps[1] = 1;
        for n in 2..=n {
            let mut st = n;
            for m in 2.. {
                if m * m > n {
                    break;
                }
                if n % m == 0 {
                    let m2 = n / m;
                    st = st.min(steps[m] + m2).min(steps[m2] + m);
                }
            }
            steps[n] = st;
        }
        steps[n] as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_3() {
        assert_eq!(Solution::min_steps(3), 3);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::min_steps(1000));
    }
}
