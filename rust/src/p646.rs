/*
Problem 646. Maximum Length of Pair Chain
=========================================

https://leetcode.com/problems/maximum-length-of-pair-chain/

You are given n pairs of numbers. In every pair, the first number is always smaller than the second
number.

Now, we define a pair (c, d) can follow another pair (a, b) if and only if b < c. Chain of pairs
can be formed in this fashion.

Given a set of pairs, find the length longest chain which can be formed. You needn't use up all the
given pairs. You can select pairs in any order.

Example 1:
    Input: [[1,2], [2,3], [3,4]]
    Output: 2
    Explanation: The longest chain is [1,2] -> [3,4]

Note:
    The number of given pairs will be in the range [1, 1000].
*/

impl Solution {
    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        // greedy search
        let mut pairs = pairs;
        pairs.sort_unstable();
        let mut cnt = 0;
        let mut tail = std::i32::MIN;
        for pair in pairs {
            if pair[0] > tail {
                // the new entry could be picked
                tail = pair[1];
                cnt += 1;
            } else if pair[1] < tail {
                // the lastest chained entry could be replaced (since sorted)
                tail = pair[1];
            }
        }
        cnt
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_chain1() {
        assert_eq!(
            Solution::find_longest_chain(vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
            2
        );
    }

    #[test]
    fn test_chain2() {
        assert_eq!(
            Solution::find_longest_chain(vec![
                vec![-10, -8],
                vec![8, 9],
                vec![-5, 0],
                vec![6, 10],
                vec![-6, -4],
                vec![1, 7],
                vec![9, 10],
                vec![-4, 7]
            ]),
            4
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::find_longest_chain(vec![vec![1, 2], vec![2, 3], vec![3, 4]]));
    }
}
