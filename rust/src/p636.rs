/*
Problem 636. Exclusive Time of Functions
========================================

https://leetcode.com/problems/exclusive-time-of-functions/

On a single-threaded CPU, we execute a program containing n functions. Each function has a unique
ID between 0 and n-1.

Function calls are stored in a call stack: when a function call starts, its ID is pushed onto the
stack, and when a function call ends, its ID is popped off the stack. The function whose ID is at
the top of the stack is the current function being executed. Each time a function starts or ends,
we write a log with the ID, whether it started or ended, and the timestamp.

You are given a list logs, where logs[i] represents the ith log message formatted as a string
"{function_id}:{"start" | "end"}:{timestamp}". For example, "0:start:3" means a function call with
function ID 0 started at the beginning of timestamp 3, and "1:end:2" means a function call with
function ID 1 ended at the end of timestamp 2. Note that a function can be called multiple times,
possibly recursively.

A function's exclusive time is the sum of execution times for all function calls in the program.
For example, if a function is called twice, one call executing for 2 time units and another call
executing for 1 time unit, the exclusive time is 2 + 1 = 3.

Return the exclusive time of each function in an array, where the value at the ith index represents
the exclusive time for the function with ID i.

Example 1:

    https://assets.leetcode.com/uploads/2019/04/05/diag1b.png

    Input: n = 2, logs = ["0:start:0","1:start:2","1:end:5","0:end:6"]
    Output: [3,4]
    Explanation:
    - Function 0 starts at the beginning of time 0, then it executes 2 for units of time and
      reaches the end of time 1.
    - Function 1 starts at the beginning of time 2, executes for 4 units of time, and ends at the
      end of time 5.
    - Function 0 resumes execution at the beginning of time 6 and executes for 1 unit of time.
    So function 0 spends 2 + 1 = 3 units of total time executing, and function 1 spends 4 units of total time executing.

Example 2:

    Input: n = 1, logs = ["0:start:0","0:start:2","0:end:5","0:start:6","0:end:6","0:end:7"]
    Output: [8]
    Explanation:
    - Function 0 starts at the beginning of time 0, executes for 2 units of time, and recursively
      calls itself.
    - Function 0 (recursive call) starts at the beginning of time 2 and executes for 4 units of
      time.
    - Function 0 (initial call) resumes execution then immediately calls itself again.
    - Function 0 (2nd recursive call) starts at the beginning of time 6 and executes for 1 unit of time.
    - Function 0 (initial call) resumes execution at the beginning of time 7 and executes for 1
      unit of time.
    So function 0 spends 2 + 4 + 1 + 1 = 8 units of total time executing.

Example 3:

    Input: n = 2, logs = ["0:start:0","0:start:2","0:end:5","1:start:6","1:end:6","0:end:7"]
    Output: [7,1]
    Explanation:
    - Function 0 starts at the beginning of time 0, executes for 2 units of time, and recursively
      calls itself.
    - Function 0 (recursive call) starts at the beginning of time 2 and executes for 4 units of
      time.
    - Function 0 (initial call) resumes execution then immediately calls function 1.
    - Function 1 starts at the beginning of time 6, executes 1 units of time, and ends at the end
      of time 6.
    - Function 0 resumes execution at the beginning of time 6 and executes for 2 units of time.
    So function 0 spends 2 + 4 + 1 = 7 units of total time executing, and function 1 spends 1 unit
    of total time executing.

Example 4:

    Input: n = 2, logs = ["0:start:0","0:start:2","0:end:5","1:start:7","1:end:7","0:end:8"]
    Output: [8,1]

Example 5:

    Input: n = 1, logs = ["0:start:0","0:end:0"]
    Output: [1]

Constraints:

    - 1 <= n <= 100
    - 1 <= logs.length <= 500
    - 0 <= function_id < n
    - 0 <= timestamp <= 10^9
    - No two start events will happen at the same timestamp.
    - No two end events will happen at the same timestamp.
    - Each function has an "end" log for each "start" log.
*/

impl Solution {
    fn parse_record(rec: &str) -> (usize, &str, i32) {
        let mut parts = rec.split(':');
        let func = parts.next().unwrap().parse().unwrap();
        let action = parts.next().unwrap();
        let ts = parts.next().unwrap().parse().unwrap();
        (func, action, ts)
    }

    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut result = vec![0; n as usize];
        let mut callstack = Vec::with_capacity(250);
        let mut cur_start = 0;
        for rec in logs.into_iter() {
            let (func, action, ts) = Self::parse_record(&rec);
            match action {
                "start" => {
                    if let Some(&cur_func) = callstack.last() {
                        result[cur_func] += ts - cur_start;
                    }
                    callstack.push(func);
                    cur_start = ts;
                }
                "end" => {
                    if let Some(cur_func) = callstack.pop() {
                        result[cur_func] += ts + 1 - cur_start;
                    }
                    cur_start = ts + 1;
                }
                _ => unreachable!(),
            }
        }
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
        assert_eq!(
            Solution::exclusive_time(
                2,
                vec!["0:start:0", "1:start:2", "1:end:5", "0:end:6"]
                    .into_iter()
                    .map(String::from)
                    .collect()
            ),
            vec![3, 4]
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::exclusive_time(
                1,
                vec![
                    "0:start:0",
                    "0:start:2",
                    "0:end:5",
                    "0:start:6",
                    "0:end:6",
                    "0:end:7"
                ]
                .into_iter()
                .map(String::from)
                .collect()
            ),
            vec![8]
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::exclusive_time(
                2,
                vec![
                    "0:start:0",
                    "0:start:2",
                    "0:end:5",
                    "1:start:6",
                    "1:end:6",
                    "0:end:7"
                ]
                .into_iter()
                .map(String::from)
                .collect()
            ),
            vec![7, 1]
        );
    }

    #[test]
    fn test_case4() {
        assert_eq!(
            Solution::exclusive_time(
                2,
                vec![
                    "0:start:0",
                    "0:start:2",
                    "0:end:5",
                    "1:start:7",
                    "1:end:7",
                    "0:end:8"
                ]
                .into_iter()
                .map(String::from)
                .collect()
            ),
            vec![8, 1]
        );
    }

    #[test]
    fn test_case5() {
        assert_eq!(
            Solution::exclusive_time(
                1,
                vec!["0:start:0", "0:end:0"]
                    .into_iter()
                    .map(String::from)
                    .collect()
            ),
            vec![1]
        );
    }

    #[test]
    fn test_case6() {
        assert_eq!(
            Solution::exclusive_time(
                3,
                vec![
                    "0:start:0",
                    "0:end:0",
                    "1:start:1",
                    "1:end:1",
                    "2:start:2",
                    "2:end:2",
                    "2:start:3",
                    "2:end:3"
                ]
                .into_iter()
                .map(String::from)
                .collect()
            ),
            vec![1, 1, 2]
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::exclusive_time(0, Vec::new()), Vec::new());
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::exclusive_time(
                2,
                vec!["0:start:0", "1:start:2", "1:end:5", "0:end:6"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
            )
        });
    }
}
