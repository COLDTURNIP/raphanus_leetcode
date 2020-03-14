/*
Problem 18. 4Sum
=======================================

https://leetcode.com/problems/4sum/

Given array nums = [1, 0, -1, 0, -2, 2], and target = 0.

A solution set is:
[
  [-1,  0, 0, 1],
  [-2, -1, 1, 2],
  [-2,  0, 0, 2]
]

*/
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums.clone();
        nums.sort();
       let results = internal::pick_n(Vec::with_capacity(4), &nums[..], target, 4);
       results.collect()
    }
}

mod internal {
    use std::iter;

    struct NumsIter<'a>(&'a [i32]);

    impl<'a> Iterator for NumsIter<'a> {
        type Item = &'a [i32];

        fn next(&mut self) -> Option<&'a [i32]> {
            let ret = self.0;
            if ret.is_empty() {
                return None
            }
            let first = ret[0];
            loop {
                match self.0.split_first() {
                    Some((&head, tail)) => {
                        if head != first {
                            break;
                        } else {
                            self.0 = tail;
                        }
                    },
                    None => break,
                }
            }
            return Some(ret);
        }
    }

    pub fn pick_n<'a>(picked_nums: Vec<i32>, nums: &'a [i32], target: i32, to_pick: i32) -> Box<dyn Iterator<Item=Vec<i32>> + 'a> {
        //eprintln!("picked_nums {:?}, nums {:?}, target {}, to_pick {}", picked_nums, nums, target, to_pick);
        if nums.is_empty() || to_pick < 1 {
            return Box::new(iter::empty());
        }
        if to_pick == 1 {
            return if let Ok(_) = nums.binary_search(&target) {
                let mut ret = picked_nums.clone();
                ret.push(target);
                Box::new(iter::once(ret))
            } else {
                Box::new(iter::empty())
            };
        }
        return Box::new(NumsIter(nums).flat_map(move |rest: &[i32]| -> Box<dyn Iterator<Item=Vec<i32>>> {
            //eprintln!("  -> {:?}", rest);
            if rest.is_empty() {
                return Box::new(iter::empty());
            }
            let mut picked_nums = picked_nums.clone();
            let picked_n: i32 = rest[0];
            picked_nums.push(picked_n);
            return pick_n(picked_nums, &rest[1..], target-picked_n, to_pick-1);
        }));
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        let question = vec![1, 0, -1, 0, -2, 2];
        assert_eq!(Solution::four_sum(question, 0), vec![vec![-2, -1, 1, 2],
                                                         vec![-2,  0, 0, 2],
                                                         vec![-1,  0, 0, 1]]);
        let question = vec![1, -2, -5, -4, -3, 3, 3, 5];
        assert_eq!(Solution::four_sum(question, -11), vec![vec![-5, -4, -3, 1]]);
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use super::Solution;

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| Solution::four_sum(vec![1, -2, -5, -4, -3, 3, 3, 5], -11))
    }
}
