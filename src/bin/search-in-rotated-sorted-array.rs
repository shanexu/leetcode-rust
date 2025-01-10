fn main() {
    println!("{}", Solution::search(vec![1, 2, 3, 4, 5, 6, 0], 2));
    println!("{}", Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3));
    println!("{}", Solution::search(vec![2, 3, 0, 1], 2));
}

struct Solution;

use std::cmp::Ordering::{Equal, Greater, Less};
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut lo = 0;
        let mut hi = n - 1;
        while lo < hi {
            if nums[lo] < nums[hi] {
                break;
            }
            let mid = (lo + hi) / 2;
            if nums[mid] > nums[hi] {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        let mut size = n;
        let mut left = 0;
        let mut right = size;
        while left < right {
            let mid = left + size / 2;
            let mid_val = nums[(mid + lo) % n];
            let cmp = mid_val.cmp(&target);
            left = if cmp == Less { mid + 1 } else { left };
            right = if cmp == Greater { mid } else { right };
            if cmp == Equal {
                return ((mid + lo) % n) as i32;
            }
            size = right - left;
        }
        -1
    }
}
