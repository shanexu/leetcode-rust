fn main() {
    println!("{}", Solution::can_win_nim(4));
    println!("{}", Solution::can_win_nim(1));
    println!("{}", Solution::can_win_nim(2));
}

struct Solution;

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n != n >> 2 << 2
    }
}
