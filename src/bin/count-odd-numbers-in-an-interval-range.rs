fn main() {
    let high: i32 = 7;
    let low: i32 = 3;
    println!("{}", (high | low) & 1);
}

struct Solution;

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        ((high - low) >> 1) + ((low | high) & 1)
    }
}
