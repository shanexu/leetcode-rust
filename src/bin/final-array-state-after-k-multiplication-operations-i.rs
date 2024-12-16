fn main() {
    println!("{:?}", Solution::get_final_state(vec![2, 1, 3, 5, 6], 5, 2));
    println!("{:?}", Solution::get_final_state(vec![1, 2], 3, 4));
}

struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::with_capacity(nums.len());
        for (i, &v) in nums.iter().enumerate() {
            {
                heap.push(Reverse((v, i)));
            }
        }
        for _ in 0..k {
            if let Some(Reverse((v, i))) = heap.pop() {
                heap.push(Reverse((v * multiplier, i)));
            }
        }
        let mut result = vec![0; nums.len()];
        while let Some(Reverse((v, i))) = heap.pop() {
            result[i] = v;
        }
        result
    }
}
