/*
Problem 907. Sum of Subarray Minimums
=====================================

Given an array of integers arr, find the sum of min(b), where b ranges over every (contiguous)
subarray of arr. Since the answer may be large, return the answer modulo 10^9 + 7.

Example 1:

    Input: arr = [3,1,2,4]
    Output: 17
    Explanation:
    Subarrays are [3], [1], [2], [4], [3,1], [1,2], [2,4], [3,1,2], [1,2,4], [3,1,2,4].
    Minimums are 3, 1, 2, 4, 1, 1, 2, 1, 1, 1.
    Sum is 17.

Example 2:

    Input: arr = [11,81,94,43,3]
    Output: 444

Constraints:

    - 1 <= arr.length <= 3 * 10^4
    - 1 <= arr[i] <= 3 * 10^4
*/

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_007;
        fn mod_times(n: i32, times: usize) -> i32 {
            let mut ans = 0;
            let mut mask = 1_usize.rotate_right(1 + times.leading_zeros());
            while mask > 0 {
                ans = (ans << 1) % MOD;
                if mask & times != 0 {
                    ans = (ans + n) % MOD
                }
                mask >>= 1;
            }
            ans
        }

        let total_len = arr.len();
        let mut sum = 0;
        let mut seen = Vec::<(i32, usize)>::with_capacity(arr.len());
        for (i, n) in arr.into_iter().enumerate() {
            let mut seen_len = seen.len();
            for j in (0..seen_len).rev() {
                let (seen_n, idx) = seen[j];
                if seen_n < n {
                    break;
                } else {
                    let prev_less_distance = j
                        .checked_sub(1)
                        .map(|pj| idx - seen[pj].1)
                        .unwrap_or(idx + 1);
                    let next_less_distance = i - idx;
                    let partial_sum = mod_times(seen_n, prev_less_distance * next_less_distance);
                    sum = (sum + partial_sum) % MOD;
                    seen_len = j;
                }
            }
            seen.truncate(seen_len);
            seen.push((n, i));
        }
        let mut lb = 0;
        for (n, i) in seen {
            let prev_less_distance = i - lb + 1;
            let next_less_distance = total_len - i;
            let partial_sum = mod_times(n, prev_less_distance * next_less_distance);
            sum = (sum + partial_sum) % MOD;
            lb = i + 1;
        }
        sum
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(Solution::sum_subarray_mins(vec![3, 1, 2, 4]), 17);
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::sum_subarray_mins(vec![11, 81, 94, 43, 3]), 444);
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::sum_subarray_mins(vec![81, 55, 2]),
            81 + 55 * 2 + 2 * 3
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::sum_subarray_mins(vec![3, 1, 2, 4, 1, 1, 2, 1, 1, 1]));
    }
}
