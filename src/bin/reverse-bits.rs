fn main() {
    assert_eq!(0b00111001011110000010100101000000, Solution::reverse_bits(0b00000010100101000001111010011100));
}

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
