fn main() {
    println!("{}", Solution::is_power_of_two(1));
    println!("{}", Solution::is_power_of_two(16));
    println!("{}", Solution::is_power_of_two(3));
}

struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && (n & (n - 1)) == 0
    }
}
