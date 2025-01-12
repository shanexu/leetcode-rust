fn main() {
    println!("{}", Solution::largest_number(vec![432,43243]));
}

struct Solution;

use std::cmp::Ordering::Equal;
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        nums.sort_by(|left, right| {
            let left = left.as_bytes();
            let right = right.as_bytes();
            let n = left.len() + right.len();
            for i in 0..n {
                let l = if i < right.len() { right[i] } else { left[i - right.len()] };
                let r = if i < left.len() { left[i] } else { right[i - left.len()] };
                match l.cmp(&r) {
                    Equal => (),
                    non_eq => return non_eq,
                }
            }
            Equal
        });
        if nums[0] == "0".to_string() {
            return "0".to_string();
        }
        nums.join("")
    }
}
