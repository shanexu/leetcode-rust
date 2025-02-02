fn main() {}

struct Solution;

impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut ans = 0;
        for _ in 0..32 {
            ans <<= 1;
            ans |= x & 1;
            x >>= 1;
        }
        ans
    }
}
