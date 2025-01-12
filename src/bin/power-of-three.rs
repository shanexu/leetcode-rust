fn main() {}

struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        POWER_OF_THREE.binary_search(&n).is_ok()
    }
}

const POWER_OF_THREE: [i32; 20] = [1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049, 177147, 531441, 1594323, 4782969, 14348907, 43046721, 129140163, 387420489, 1162261467];

struct Solution2;

impl Solution2 {
    pub fn is_power_of_three(n: i32) -> bool {
        n > 0 && (1162261467 / n * n) == 1162261467
    }
}