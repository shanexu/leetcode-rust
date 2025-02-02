fn main() {
    assert_eq!(Solution::count_odds(3, 7), 3);
    assert_eq!(Solution::count_odds(8, 10), 1);
    assert_eq!(Solution::count_odds(8, 10), 1);
    assert_eq!(Solution::count_odds(21, 22), 1);
}

struct Solution;

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        ((high - low) >> 1) + ((low | high) & 1)
    }
}
