fn main() {
    println!("{}", Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2));
}

struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut heap = BinaryHeap::new();
        for (i, &num) in nums.iter().enumerate() {
            if i < k {
                heap.push(Reverse(num));
            } else {
                let mut peek = heap.peek_mut().unwrap();
                if peek.0 < num {
                    peek.0 = num;
                }
            }
        }
        heap.peek().unwrap().0
    }
}
