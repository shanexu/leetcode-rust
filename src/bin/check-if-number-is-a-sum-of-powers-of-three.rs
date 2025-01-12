fn main() {}

struct Solution;

impl Solution {
    pub fn check_powers_of_three(mut n: i32) -> bool {
        while n > 0 {
            match n - n / 3 * 3 {
                0 | 1 => n /= 3,
                _ => return false,
            }
        }
        true
    }
}
