fn main() {
    println!(
        "{}",
        Solution::longest_monotonic_subarray(vec![1, 4, 3, 3, 2])
    );
}

struct Solution;

use std::cmp::Ordering::{Equal, Greater, Less};
impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut prev = nums[0];
        let mut ans = 1;
        let mut inc_len = 1;
        let mut dec_len = 1;
        for &num in nums.iter().skip(1) {
            match prev.cmp(&num) {
                Less => {
                    inc_len += 1;
                    dec_len = 1;
                }
                Equal => {
                    inc_len = 1;
                    dec_len = 1;
                }
                Greater => {
                    inc_len = 1;
                    dec_len += 1;
                }
            }
            ans = ans.max(inc_len).max(dec_len);
            prev = num;
        }
        ans
    }
}
