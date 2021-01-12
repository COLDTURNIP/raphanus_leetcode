/*
Problem 406. Queue Reconstruction by Height
===========================================

https://leetcode.com/problems/queue-reconstruction-by-height/

You are given an array of people, people, which are the attributes of some people in a queue (not
necessarily in order). Each people[i] = [hi, ki] represents the ith person of height hi with
exactly ki other people in front who have a height greater than or equal to hi.

Reconstruct and return the queue that is represented by the input array people. The returned queue
should be formatted as an array queue, where queue[j] = [hj, kj] is the attributes of the jth
person in the queue (queue[0] is the person at the front of the queue).

Example 1:

    Input: people = [[7,0],[4,4],[7,1],[5,0],[6,1],[5,2]]
    Output: [[5,0],[7,0],[5,2],[6,1],[4,4],[7,1]]
    Explanation:
        - Person 0 has height 5 with no other people taller or the same height in front.
        - Person 1 has height 7 with no other people taller or the same height in front.
        - Person 2 has height 5 with two persons taller or the same height in front, which is
          person 0 and 1.
        - Person 3 has height 6 with one person taller or the same height in front, which is person
          1.
        - Person 4 has height 4 with four people taller or the same height in front, which are
          people 0, 1, 2, and 3.
        - Person 5 has height 7 with one person taller or the same height in front, which is person
          1.
        Hence [[5,0],[7,0],[5,2],[6,1],[4,4],[7,1]] is the reconstructed queue.

Example 2:

    Input: people = [[6,0],[5,0],[4,0],[3,2],[2,2],[1,4]]
    Output: [[4,0],[5,0],[2,2],[3,2],[1,4],[6,0]]

    60 50 40 32 22 14
    50 60 40 32 22 14
    40 50 60 32 22 14
    40 50 32 60 22 14
    40 50 22 32 60 14
    40 50 22 32 14 60

Constraints:

    - 1 <= people.length <= 2000
    - 0 <= hi <= 10^6
    - 0 <= ki < people.length
    - It is guaranteed that the queue can be reconstructed.
*/

impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = people;
        let len = ans.len();
        ans.sort_unstable_by(|a, b| b[0].cmp(&a[0]).then(a[1].cmp(&b[1])));
        for i in 1..len {
            let order = ans[i][1] as usize;
            ans[order..i + 1].rotate_right(1);
        }
        ans
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
            Solution::reconstruct_queue(vec![
                vec![7, 0],
                vec![4, 4],
                vec![7, 1],
                vec![5, 0],
                vec![6, 1],
                vec![5, 2],
            ]),
            vec![
                vec![5, 0],
                vec![7, 0],
                vec![5, 2],
                vec![6, 1],
                vec![4, 4],
                vec![7, 1],
            ]
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::reconstruct_queue(vec![
                vec![7, 0],
                vec![4, 4],
                vec![7, 1],
                vec![5, 0],
                vec![6, 1],
                vec![5, 2],
            ]),
            vec![
                vec![5, 0],
                vec![7, 0],
                vec![5, 2],
                vec![6, 1],
                vec![4, 4],
                vec![7, 1],
            ]
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(
            Solution::reconstruct_queue(Vec::new()),
            Vec::<Vec<i32>>::new()
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::reconstruct_queue(vec![
                vec![7, 0],
                vec![4, 4],
                vec![7, 1],
                vec![5, 0],
                vec![6, 1],
                vec![5, 2],
            ])
        });
    }
}
