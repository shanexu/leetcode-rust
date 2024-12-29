fn main() {
    Solution::arrange_coins(3);
}

struct Solution;

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let discriminant = (1.0 + 8.0 * n as f64).sqrt();
        ((-1.0 + discriminant) / 2.0).floor() as i32
    }
}