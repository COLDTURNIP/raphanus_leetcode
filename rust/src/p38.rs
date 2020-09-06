/*
Problem 38. Count and Say
==============================

https://leetcode.com/problems/count-and-say/

The count-and-say sequence is the sequence of integers with the first five terms as following:

    1.     1
    2.     11
    3.     21
    4.     1211
    5.     111221

1 is read off as "one 1" or 11.
11 is read off as "two 1s" or 21.
21 is read off as "one 2, then one 1" or 1211.

Given an integer n where 1 ≤ n ≤ 30, generate the nth term of the count-and-say sequence. You can
do so recursively, in other words from the previous member read off the digits, counting the number
of digits in groups of the same digit.

Note: Each term of the sequence of integers will be represented as a string.

Example 1:

    Input: 1
    Output: "1"

    Explanation: This is the base case.

Example 2:

    Input: 4
    Output: "1211"

    Explanation: For n = 3 the term was "21" in which we have two groups "2" and "1", "2" can be
    read as "12" which means frequency = 1 and value = 2, the same way "1" is read as "11", so the
    answer is the concatenation of "12" and "11" which is "1211".
*/

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut num = "1".to_owned();
        let mut buf = String::with_capacity(num.len() * 2);
        for _ in 1..n as usize {
            // count digits and push into buffer
            let mut iter = num.chars();
            let mut last = iter.next().unwrap();
            let mut last_cnt = 1_usize;
            for cur in iter {
                if cur == last {
                    last_cnt += 1;
                } else {
                    buf.push_str(last_cnt.to_string().as_str());
                    buf.push(last);
                    last = cur;
                    last_cnt = 1;
                }
            }
            buf.push_str(last_cnt.to_string().as_str());
            buf.push(last);

            // move number from buffer and clear buffer
            std::mem::swap(&mut num, &mut buf);
            buf.truncate(0);
        }
        num
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    extern crate test;
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::count_and_say(1), "1");
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::count_and_say(4), "1211");
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::count_and_say(10));
    }
}
