fn main() {
    println!("{}", Solution::is_power_of_four(4));
}

struct Solution;

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        POWER_OF_FOUR.binary_search(&n).is_ok()
    }
}

const POWER_OF_FOUR: &[i32] = &[
    1, 4, 16, 64, 256, 1024, 4096, 16384, 65536, 262144, 1048576, 4194304, 16777216, 67108864,
    268435456, 1073741824,
];
