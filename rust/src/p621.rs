/*
Problem 621. Task Scheduler
===========================

https://leetcode.com/problems/task-scheduler/

Given a characters array tasks, representing the tasks a CPU needs to do, where each letter
represents a different task. Tasks could be done in any order. Each task is done in one unit of
time. For each unit of time, the CPU could complete either one task or just be idle.

However, there is a non-negative integer n that represents the cooldown period between two same
tasks (the same letter in the array), that is that there must be at least n units of time between
any two same tasks.

Return the least number of units of times that the CPU will take to finish all the given tasks.

Example 1:

    Input: tasks = ["A","A","A","B","B","B"], n = 2
    Output: 8
    Explanation:
    A -> B -> idle -> A -> B -> idle -> A -> B
    There is at least 2 units of time between any two same tasks.

Example 2:

    Input: tasks = ["A","A","A","B","B","B"], n = 0
    Output: 6
    Explanation: On this case any permutation of size 6 would work since n = 0.
    ["A","A","A","B","B","B"]
    ["A","B","A","B","A","B"]
    ["B","B","B","A","A","A"]
    ...
    And so on.

Example 3:

    Input: tasks = ["A","A","A","A","A","A","B","C","D","E","F","G"], n = 2
    Output: 16
    Explanation:
    One possible solution is
    A -> B -> C -> A -> D -> E -> A -> F -> G -> A -> idle -> idle -> A -> idle -> idle -> A

Constraints:

    - 1 <= task.length <= 10^4
    - tasks[i] is upper-case English letter.
    - The integer n is in the range [0, 100].
*/

use std::cmp::Ordering::{Equal, Greater, Less};

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let min_schedule = tasks.len();
        if min_schedule == 0 {
            return 0;
        }
        let (max_task, max_task_kind) = {
            let mut cnt = vec![0; 26];
            let (mut max_task, mut max_task_kind) = (0, 0);
            for tc in tasks {
                let mut tb = [0; 1];
                tc.encode_utf8(&mut tb);
                let c = &mut cnt[(tb[0] - b'A') as usize];
                *c += 1;
                match max_task.cmp(c) {
                    Less => {
                        max_task = *c;
                        max_task_kind = 1;
                    }
                    Equal => max_task_kind += 1,
                    Greater => {}
                }
            }
            (max_task, max_task_kind)
        };

        // schedule the tasks which appears max_task times, then fill the empty slots.
        let schedule_seg = max_task - 1;
        let schedule_seg_len = (n as usize).saturating_sub(max_task_kind - 1);
        let rest_empty_slot = schedule_seg * schedule_seg_len;
        let unscheduled = min_schedule - max_task * max_task_kind;
        let idle_slot = rest_empty_slot.saturating_sub(unscheduled);
        (min_schedule + idle_slot) as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(Solution::least_interval("AAABBB".chars().collect(), 2), 8);
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::least_interval("AAABBB".chars().collect(), 0), 6);
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::least_interval("AAAAAABCDEFG".chars().collect(), 2),
            16
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::least_interval(Vec::new(), 9), 0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::least_interval("AAAAAABCDEFG".chars().collect(), 2));
    }
}
