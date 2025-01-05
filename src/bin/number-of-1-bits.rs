fn main() {
    println!(
        "{}",
        Solution::hamming_weight(0b00000000000000000000000000001011)
    );
    println!(
        "{}",
        Solution::hamming_weight(0b00000000000000000000000010000000)
    );
}

struct Solution;

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut n = n;
        let mut count = 0;
        while n != 0 {
            count += n & 1;
            n >>= 1;
        }
        count
    }
}
