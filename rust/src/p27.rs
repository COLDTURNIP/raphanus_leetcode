
/*
Problem 27. Remove Element
=======================================

https://leetcode.com/problems/remove-element/

*/
#[allow(dead_code)]

pub fn solution(nums: &mut Vec<i32>, val: i32) -> i32 {
    remove_element(nums, val)
}

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut cur: usize = 0;
    for idx in 0..nums.len() {
        let cur_val = nums[idx];
        if cur_val != val {
            nums[cur] = cur_val;
            cur += 1;
        }
    }
    cur as i32
}
