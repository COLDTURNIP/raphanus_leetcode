/*
Problem 605. Can Place Flowers
==============================

https://leetcode.com/problems/can-place-flowers/

Suppose you have a long flowerbed in which some of the plots are planted and some are not. However,
flowers cannot be planted in adjacent plots - they would compete for water and both would die.

Given a flowerbed (represented as an array containing 0 and 1, where 0 means empty and 1 means not
empty), and a number n, return if n new flowers can be planted in it without violating the
no-adjacent-flowers rule.

Example 1:

    Input: flowerbed = [1,0,0,0,1], n = 1
    Output: True

Example 2:

    Input: flowerbed = [1,0,0,0,1], n = 2
    Output: False

Note:
    - The input array won't violate no-adjacent-flowers rule.
    - The input array size is in the range of [1, 20000].
    - n is a non-negative integer which won't exceed the input array size.
*/

impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        if n < 0 {
            return false;
        } else if flowerbed.is_empty() {
            return n == 0;
        } else if n == 0 {
            return true;
        }
        let len = flowerbed.len();
        match (flowerbed.get(0), flowerbed.get(1)) {
            (Some(0), None) | (Some(0), Some(0)) => {
                flowerbed[0] = 1;
                n -= 1;
            }
            _ => {}
        }
        match (
            len.checked_sub(2).and_then(|i| flowerbed.get(i)),
            len.checked_sub(1).and_then(|i| flowerbed.get(i)),
        ) {
            (None, Some(0)) | (Some(0), Some(0)) => {
                flowerbed[len - 1] = 1;
                n -= 1;
            }
            _ => {}
        }
        for i in 0..len.saturating_sub(2) {
            match flowerbed.get_mut(i..i + 3) {
                Some(&mut [0, ref mut m, 0]) if *m == 0 => {
                    *m = 1;
                    n -= 1;
                }
                _ => {}
            }
            if n <= 0 {
                break;
            }
        }
        n <= 0
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_10001_1() {
        assert!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1));
    }

    #[test]
    fn test_10001_2() {
        assert!(!Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2));
    }

    #[test]
    fn test_1_1() {
        assert!(!Solution::can_place_flowers(vec![1], 1));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1));
    }
}
