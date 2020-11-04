/*
Problem 985. Sum of Even Numbers After Queries
==============================================

https://leetcode.com/problems/sum-of-even-numbers-after-queries/

We have an array A of integers, and an array queries of queries.

For the i-th query val = queries[i][0], index = queries[i][1], we add val to A[index].  Then, the
answer to the i-th query is the sum of the even values of A.

(Here, the given index = queries[i][1] is a 0-based index, and each query permanently modifies the
array A.)

Return the answer to all queries.  Your answer array should have answer[i] as the answer to the
i-th query.

Example 1:

    Input: A = [1,2,3,4], queries = [[1,0],[-3,1],[-4,0],[2,3]]
    Output: [8,6,2,4]
    Explanation:
        - At the beginning, the array is [1,2,3,4].
        - After adding 1 to A[0], the array is [2,2,3,4], and the sum of even values is 2 + 2 + 4 = 8.
        - After adding -3 to A[1], the array is [2,-1,3,4], and the sum of even values is 2 + 4 = 6.
        - After adding -4 to A[0], the array is [-2,-1,3,4], and the sum of even values is -2 + 4 = 2.
        - After adding 2 to A[3], the array is [-2,-1,3,6], and the sum of even values is -2 + 6 = 4.

Note:

    - 1 <= A.length <= 10000
    - -10000 <= A[i] <= 10000
    - 1 <= queries.length <= 10000
    - -10000 <= queries[i][0] <= 10000
    - 0 <= queries[i][1] < A.length
*/

impl Solution {
    pub fn sum_even_after_queries(mut a: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let sum = a.iter().filter(|&n| n & 1 == 0).sum();
        queries
            .into_iter()
            .scan(sum, |sum, change| {
                let (diff, i) = (change[0], change[1] as usize);
                if a[i] & 1 == 0 {
                    *sum -= a[i];
                }
                a[i] += diff;
                if a[i] & 1 == 0 {
                    *sum += a[i];
                }
                Some(*sum)
            })
            .collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_sum_query() {
        assert_eq!(
            Solution::sum_even_after_queries(
                vec![1, 2, 3, 4],
                vec![vec![1, 0], vec![-3, 1], vec![-4, 0], vec![2, 3]]
            ),
            vec![8, 6, 2, 4]
        )
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::sum_even_after_queries(
                vec![1, 2, 3, 4],
                vec![vec![1, 0], vec![-3, 1], vec![-4, 0], vec![2, 3]],
            )
        });
    }
}
