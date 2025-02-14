use std::i32;

fn main() {
    println!("{}", Solution::partition_disjoint(vec![5, 0, 3, 8, 6]));
    println!("{}", Solution::partition_disjoint(vec![1, 1, 1, 0, 6, 12]));
}

struct Solution;

impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut min = vec![0; n];
        min[n - 1] = nums[n - 1];
        for i in (0..n - 1).rev() {
            min[i] = min[i + 1].min(nums[i])
        }
        let mut max = i32::MIN;
        for i in 0..n - 1 {
            max = max.max(nums[i]);
            if max <= min[i + 1] {
                return (i + 1) as _;
            }
        }
        0
    }
}
