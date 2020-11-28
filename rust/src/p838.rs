/*
Problem 838. Push Dominoes
==========================

https://leetcode.com/problems/push-dominoes/

There are N dominoes in a line, and we place each domino vertically upright.

In the beginning, we simultaneously push some of the dominoes either to the left or to the right.

https://s3-lc-upload.s3.amazonaws.com/uploads/2018/05/18/domino.png

After each second, each domino that is falling to the left pushes the adjacent domino on the left.

Similarly, the dominoes falling to the right push their adjacent dominoes standing on the right.

When a vertical domino has dominoes falling on it from both sides, it stays still due to the
balance of the forces.

For the purposes of this question, we will consider that a falling domino expends no additional
force to a falling or already fallen domino.

Given a string "S" representing the initial state. S[i] = 'L', if the i-th domino has been pushed
to the left; S[i] = 'R', if the i-th domino has been pushed to the right; S[i] = '.', if the i-th
domino has not been pushed.

Return a string representing the final state.

Example 1:

    Input: ".L.R...LR..L.."
    Output: "LL.RR.LLRRLL.."

Example 2:

    Input: "RR.L"
    Output: "RR.L"
    Explanation: The first domino expends no additional force on the second domino.

Note:

    - 0 <= N <= 10^5
    - String dominoes contains only 'L', 'R' and '.'
*/

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut prev_state: Vec<char> = dominoes.chars().collect();
        let mut cur_state = vec!['.'; prev_state.len()];
        let mut changed = true;
        while changed {
            changed = false;
            for (i, domino) in cur_state.iter_mut().enumerate() {
                let prev = prev_state[i];
                if prev != '.' {
                    *domino = prev;
                    continue;
                }
                let left = i
                    .checked_sub(1)
                    .and_then(|i| prev_state.get(i))
                    .cloned()
                    .unwrap_or('.');
                let right = prev_state.get(i + 1).cloned().unwrap_or('.');
                *domino = match (left, right) {
                    ('R', 'L') => '.',
                    ('R', _) => 'R',
                    (_, 'L') => 'L',
                    _ => '.',
                };
                changed |= *domino != prev;
            }
            std::mem::swap(&mut prev_state, &mut cur_state)
        }
        cur_state.into_iter().collect()
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
            Solution::push_dominoes(".L.R...LR..L..".to_owned()),
            "LL.RR.LLRRLL..".to_owned()
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::push_dominoes("RR.L".to_owned()),
            "RR.L".to_owned()
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::push_dominoes(String::new()), String::new());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::push_dominoes(".L.R...LR..L..".to_owned()));
    }
}
