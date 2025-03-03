fn main() {
    println!(
        "{:?}",
        Solution::pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10)
    );
}

struct Solution;

use std::cmp::Ordering::{Greater, Less};
impl Solution {
    pub fn pivot_array(mut nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let n = nums.len();
        let mut j = 0;
        let mut gt = vec![];
        for i in 0..n {
            let num = nums[i];
            match num.cmp(&pivot) {
                Less => {
                    nums[j] = num;
                    j += 1;
                }
                Greater => gt.push(num),
                _ => (),
            }
        }
        for i in j..n - gt.len() {
            nums[i] = pivot;
        }
        for i in 0..gt.len() {
            nums[n - gt.len() + i] = gt[i];
        }
        nums
    }
}
