fn main() {
    println!("{}", Solution::is_perfect_square(2147483647));
}

struct Solution;

use std::cmp::Ordering::{Equal, Greater, Less};
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let dr = (num - 1) % 9 + 1;
        if dr != 1 && dr != 4 && dr != 7 && dr != 9 {
            return false;
        }
        let num = num as i64;
        let mut left = 1;
        let mut right = num;
        while left <= right {
            let mid = (left + right) / 2;
            let mid_square = mid * mid;
            match mid_square.cmp(&num) {
                Less => left = mid + 1,
                Greater => right = mid - 1,
                Equal => return true,
            }
        }
        false
    }
}
