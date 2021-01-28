/*
Problem 735. Asteroid Collision
===============================

https://leetcode.com/problems/asteroid-collision/

We are given an array asteroids of integers representing asteroids in a row.

For each asteroid, the absolute value represents its size, and the sign represents its direction
(positive meaning right, negative meaning left). Each asteroid moves at the same speed.

Find out the state of the asteroids after all collisions. If two asteroids meet, the smaller one
will explode. If both are the same size, both will explode. Two asteroids moving in the same
direction will never meet.

Example 1:

    Input: asteroids = [5,10,-5]
    Output: [5,10]
    Explanation: The 10 and -5 collide resulting in 10.  The 5 and 10 never collide.

Example 2:

    Input: asteroids = [8,-8]
    Output: []
    Explanation: The 8 and -8 collide exploding each other.

Example 3:

    Input: asteroids = [10,2,-5]
    Output: [10]
    Explanation: The 2 and -5 collide resulting in -5. The 10 and -5 collide resulting in 10.

Example 4:

    Input: asteroids = [-2,-1,1,2]
    Output: [-2,-1,1,2]
    Explanation: The -2 and -1 are moving left, while the 1 and 2 are moving right. Asteroids
                 moving the same direction never meet, so no asteroids will meet each other.

Constraints:

    - 1 <= asteroids <= 10^4
    - -1000 <= asteroids[i] <= 1000
    - asteroids[i] != 0
*/

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        // core idea: reuse left side of asteroids as a stack
        let mut result = asteroids;
        let mut checked_len: usize = 0;
        for i in 0..result.len() {
            let cur_move_size = result[i];
            let insert_size = loop {
                match (cur_move_size > 0, checked_len.checked_sub(1)) {
                    (true, _) | (_, None) => break cur_move_size,
                    (false, Some(left_idx)) => {
                        let cur_size = -cur_move_size;
                        let left_size = result[left_idx];
                        if left_size < 0 {
                            break cur_move_size;
                        } else if left_size == cur_size {
                            checked_len = left_idx;
                            break 0;
                        } else if left_size > cur_size {
                            break 0;
                        }
                    }
                }
                checked_len -= 1;
            };
            if insert_size != 0 {
                result[checked_len] = insert_size;
                checked_len += 1;
            }
            //println!("{:?}", &result[..checked_len]);
        }
        result.truncate(checked_len);
        result
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(Solution::asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::asteroid_collision(vec![8, -8]), Vec::new());
    }

    #[test]
    fn test_case3() {
        assert_eq!(Solution::asteroid_collision(vec![10, 2, -5]), vec![10]);
    }

    #[test]
    fn test_case4() {
        assert_eq!(
            Solution::asteroid_collision(vec![-2, -1, 1, 2]),
            vec![-2, -1, 1, 2]
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::asteroid_collision(Vec::new()), Vec::new());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::asteroid_collision(vec![5, 10, -5, -99, 62, 272, -3, 478]));
    }
}
