fn main() {}

struct Solution;

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n != n >> 2 << 2
    }
}