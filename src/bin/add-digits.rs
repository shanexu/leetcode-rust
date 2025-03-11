fn main() {
    assert_eq!(2, Solution::add_digits(38));
}

struct Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        (num - 1) % 9 + 1
    }
}
