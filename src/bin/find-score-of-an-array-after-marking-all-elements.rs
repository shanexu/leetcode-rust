fn main() {
    println!("{}", Solution::find_score(vec![2, 1, 3, 4, 5, 2]));
    println!("{}", Solution::find_score(vec![2, 3, 5, 1, 3, 2]));
}

struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let mut heap = BinaryHeap::new();
        for (i, &v) in nums.iter().enumerate() {
            heap.push(Reverse((v, i)));
        }
        let mut marked = vec![false; nums.len()];
        let mut sum = 0;
        while let Some(Reverse((v, i))) = heap.pop() {
            if marked[i] {
                continue;
            }
            sum += v as i64;
            marked[i] = true;
            if i > 0 {
                marked[i - 1] = true;
            }
            if i < nums.len() - 1 {
                marked[i + 1] = true;
            }
        }
        sum
    }
}
