fn main() {
    println!("{}", Solution::min_operations(vec![2, 11, 10, 1, 3], 10));
}

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().filter(|&x| *x < k).count() as i32
    }
}
